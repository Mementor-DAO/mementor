use std::{env, fs, path::{Path, PathBuf}, sync::RwLock};
use bot_api::insert_image::{ImageInsertRequest, ImageInsertResponse};
use ic_agent::{export::Principal, identity::BasicIdentity, Agent};
use ic_utils::Canister;

static AGENT: RwLock<Option<Agent>> = RwLock::new(None);

#[tokio::main]
async fn main(
) {
    let args: Vec<String> = env::args().collect();
    let bot_canister_id = Principal::from_text(&args[1]).unwrap();
    let pem_file_path = &args[2];
    
    setup(pem_file_path)
        .await.unwrap();

    let files = collect_files();

    upload_files(bot_canister_id, files)
        .await.unwrap();
}

async fn setup(
    pem_file_path: &str
) -> Result<(), String> {
    let agent = create_agent("https://icp0.io", true, pem_file_path)
        .await
        .map_err(|err| err.to_string())?;
    *AGENT.write().unwrap() = Some(agent);
    
    Ok(())
}

async fn upload_files(
    bot_canister_id: Principal,
    files: Vec<PathBuf>
) -> Result<(), String> {
    let agent = AGENT.read().unwrap().clone().unwrap();

    let canister = Canister::builder()
        .with_agent(&agent)
        .with_canister_id(bot_canister_id)
        .build()
        .map_err(|err| err.to_string())?;

    for fnames in files.chunks(8) {
        let mut futs = vec![];
        for fname in fnames {
            let s_id = fname.file_name().unwrap().to_string_lossy()
                .replace(".jpg", "");
            let id = u32::from_str_radix(&s_id, 10).unwrap();

            println!("Uploading id({}) {}", id, fname.to_string_lossy());

            let jpg = fs::read(fname).unwrap();

            futs.push(upload_file(&canister, id, jpg));
        }
        
        futures::future::join_all(futs).await;
    }

    Ok(())
}

async fn upload_file(
    canister: &Canister<'_>, 
    id: u32, 
    data: Vec<u8>
) -> Result<(), String> {
    let (_, ): (ImageInsertResponse, ) = canister.update("insert_image")
        .with_arg(ImageInsertRequest { 
            id, 
            mime_type: "image/jpeg".to_string(), 
            data
        })
        .build()
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

async fn create_agent(
    url: &str, 
    is_mainnet: bool,
    pem_file_path: &str
) -> Result<Agent, String> {
    let agent = Agent::builder()
        .with_url(url)
        .with_identity(
            BasicIdentity::from_pem_file(
                Path::new(pem_file_path)
            ).unwrap()
        )
        .build()
        .map_err(|err| err.to_string())?;
    
    if !is_mainnet {
        agent.fetch_root_key()
            .await
            .map_err(|err| err.to_string())?;
    }
    
    Ok(agent)
}

fn collect_files(
) -> Vec<PathBuf> {
    let mut files = vec![];
    
    for entry in glob::glob("./packages/assets/images/*.jpg")
        .expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                files.push(path);
            },
            Err(e) => println!("{:?}", e),
        }
    }

    files
}


use std::{io::{self, ErrorKind}, ops::Range, path::Path, sync::Arc};
use tantivy::{
    directory::{
        error, DirectoryLock, FileHandle, FileSlice, 
        Lock, OwnedBytes, WatchCallback, WatchHandle, WritePtr
    }, 
    Directory, 
    HasLen
};
use tar_no_std::{ArchiveEntry, TarArchive};

#[derive(Debug, Clone)]
pub struct TarDirectory {
    arc: TarArchive,
    root: String,
}

struct MockLock {
}

#[derive(Debug)]
pub struct TarFileHandle {
    data: Vec<u8>,
}

impl HasLen for TarFileHandle {
    fn len(
        &self
    ) -> usize {
        self.data.len()
    }
}

impl FileHandle for TarFileHandle {
    fn read_bytes(
        &self,
        range: Range<usize>
    ) -> io::Result<OwnedBytes>  {
        Ok(OwnedBytes::new(self.data.as_slice()[range].to_vec()))
    }
}

impl TarDirectory {
    pub fn from_bytes(
        bytes: Box<[u8]>,
        root: String
    ) -> Result<Self, String> {
        let arc = TarArchive::new(bytes).map_err(|e| e.to_string())?;
        Ok(Self {
            arc,
            root,
        })
    }

    fn _find_entry<'a>(
        &'a self,
        path: &Path
    ) -> Option<ArchiveEntry<'a>> {

        let path = self.root.clone() + &path.to_string_lossy().to_string();
        
        for entry in self.arc.entries() {
            if entry.filename().as_str().unwrap() == path {
                return Some(entry);
            }
        }

        None
    }
}

impl Directory for TarDirectory {
    fn get_file_handle(
        &self, 
        path: &Path
    ) -> Result<std::sync::Arc<dyn FileHandle>, error::OpenReadError> {
        match self._find_entry(path) {
            Some(e) => {
                Ok(Arc::new(TarFileHandle{
                    data: e.data().to_vec()
                }))
            },
            None => {
                Err(error::OpenReadError::FileDoesNotExist(path.to_path_buf()))
            }
        }
    }

    fn delete(
        &self, 
        path: &Path
    ) -> Result<(), error::DeleteError> {
        Err(error::DeleteError::IoError { 
            io_error: Arc::new(std::io::Error::new(ErrorKind::Unsupported, "Unsupported")), 
            filepath: path.to_path_buf() 
        })
    }

    fn exists(
        &self, 
        path: &Path
    ) -> Result<bool, error::OpenReadError> {
        match self._find_entry(path) {
            Some(_) => {
                Ok(true)
            },
            None => {
                Ok(false)
            }
        }
    }

    fn open_write(
        &self, 
        path: &Path
    ) -> Result<WritePtr, error::OpenWriteError> {
        Err(error::OpenWriteError::IoError { 
            io_error: Arc::new(std::io::Error::new(ErrorKind::Unsupported, "Unsupported")), 
            filepath: path.to_path_buf() 
        })
    }

    fn atomic_read(
        &self, 
        path: &Path
    ) -> Result<Vec<u8>, error::OpenReadError> {
        match self._find_entry(path) {
            Some(e) => {
                Ok(e.data().to_vec())
            },
            None => {
                Err(error::OpenReadError::FileDoesNotExist(path.to_path_buf()))
            }
        }
    }

    fn atomic_write(
        &self, 
        _path: &Path, 
        _data: &[u8]
    ) -> std::io::Result<()> {
        Ok(())
    }

    fn sync_directory(
        &self
    ) -> std::io::Result<()> {
        Ok(())
    }

    fn watch(
        &self, 
        watch_callback: WatchCallback
    ) -> tantivy::Result<WatchHandle> {
        Ok(WatchHandle::new(Arc::new(watch_callback)))
    }
    
    fn open_read(
        &self, 
        path: &Path
    ) -> Result<FileSlice, error::OpenReadError> {
        let file_handle = self.get_file_handle(path)?;
        Ok(FileSlice::new(file_handle))
    }
    
    fn acquire_lock(
        &self, 
        _lock: &Lock
    ) -> Result<DirectoryLock, error::LockError> {
        Ok(DirectoryLock::from(DirectoryLock::from(Box::new(MockLock{}))))
    }
}
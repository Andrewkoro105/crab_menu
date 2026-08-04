use crate::app::{
    element::{Diagram, Element},
    functions::{get::Get, run::BoxedRun},
};
use std::path::{Path, PathBuf};
use tracing::{debug, error};

#[derive(Clone, Hash)]
pub struct Files<R: BoxedRun> {
    dir: PathBuf,
    files: Option<Vec<FileData>>,
    run: fn(&FileData) -> R,
}

#[derive(Clone, Default, Hash)]
pub struct FileData {
    pub path: PathBuf,
}

#[derive(Clone, Default, Hash)]
pub struct ListData {
    pub filter: String,
}

impl<R: BoxedRun> Files<R> {
    pub fn new(dir: PathBuf, fixed: bool, run: fn(&FileData) -> R) -> Self {
        Self {
            files: fixed.then(|| Self::load_files_data(&dir)),
            dir,
            run,
        }
    }

    fn load_files_data(dir: &Path) -> Vec<FileData> {
        let read_dir_result = dir.read_dir();

        match read_dir_result {
            Ok(read_dir) => {
                let mut result = read_dir
                    .into_iter()
                    .filter_map(|dir_entry_result| match dir_entry_result {
                        Ok(dir_entre) => Some(FileData {
                            path: dir_entre.path(),
                        }),
                        Err(err) => {
                            error!("Error retrieving a file inside a folder: {dir:?}: {err}");
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                result.sort_by(|a, b| a.path.cmp(&b.path));
                result
            }
            Err(err) => {
                error!("Unable to open the folder: {dir:?}: {err}");
                vec![]
            }
        }
    }

    fn get_files_data(&self) -> Vec<FileData> {
        self.files
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Self::load_files_data(&self.dir))
    }
}

impl<R: BoxedRun> Get<R, FileData, FileData, String> for Files<R> {
    async fn get(
        &self,
        find_data: &String,
        _id: usize,
        start: usize,
        target_count: usize,
    ) -> Vec<Diagram<R, FileData, FileData>> {
        self.get_files_data()
            .into_iter()
            .filter_map(|data| {
                let file_name = data
                    .path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                (find_data.is_empty() || file_name
                    .starts_with(find_data))
                    .then_some(Diagram::Element(Element {
                        run: (self.run)(&data),
                        additional_data: data,
                    }))
            })
            .skip(start)
            .take(target_count)
            .collect()
    }
}

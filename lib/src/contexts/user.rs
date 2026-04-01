use crate::contexts::{Context, ContextProvider};
use anyhow::Result;

pub struct UserContextProvider {}

impl ContextProvider for UserContextProvider {
    fn get_prefix(&self) -> String {
        String::from("user")
    }

    fn get_contexts(&self) -> Result<Vec<super::Context>> {
        let name = whoami::realname().unwrap_or_else(|_| String::from("unknown"));
        let username = whoami::username().unwrap_or_else(|_| String::from("unknown"));

        let mut dirs: Vec<super::Context> = vec![
            ("audio_dir", dirs_next::audio_dir()),
            ("cache_dir", dirs_next::cache_dir()),
            ("config_dir", dirs_next::config_dir()),
            ("data_dir", dirs_next::data_dir()),
            ("data_local_dir", dirs_next::data_local_dir()),
            ("desktop_dir", dirs_next::desktop_dir()),
            ("document_dir", dirs_next::document_dir()),
            ("download_dir", dirs_next::download_dir()),
            ("executable_dir", dirs_next::executable_dir()),
            ("font_dir", dirs_next::font_dir()),
            ("home_dir", dirs_next::home_dir()),
            ("picture_dir", dirs_next::picture_dir()),
            ("public_dir", dirs_next::public_dir()),
            ("runtime_dir", dirs_next::runtime_dir()),
            ("template_dir", dirs_next::template_dir()),
            ("video_dir", dirs_next::video_dir()),
        ]
        .iter()
        .map(|(key, value)| {
            Context::KeyValueContext(
                key.to_string(),
                value
                    .clone()
                    .map(Into::into)
                    .unwrap_or_else(|| "unknown".into()),
            )
        })
        .collect();
        dirs.append(&mut vec![
            Context::KeyValueContext(String::from("id"), self.get_uid().to_string().into()),
            Context::KeyValueContext(String::from("name"), name.into()),
            Context::KeyValueContext(String::from("username"), username.into()),
        ]);

        Ok(dirs)
    }
}

impl UserContextProvider {
    #[cfg(unix)]
    fn get_uid(&self) -> u32 {
        uzers::get_current_uid()
    }

    #[cfg(not(unix))]
    fn get_uid(&self) -> u32 {
        0
    }
}

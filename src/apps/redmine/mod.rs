use crate::{Action, State};
use thirtyfour::prelude::*;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            st.wd.goto(st.url.as_str()).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-mainpage.png"))
                .await?;
            (st.wd.find(By::ClassName("login")).await?).click().await?;
            (st.wd.find(By::Id("username")).await?)
                .send_keys("admin")
                .await?;
            (st.wd.find(By::Id("password")).await?)
                .send_keys(&st.pse.app_pass)
                .await?;
            (st.wd.find(By::Id("login-submit")).await?).click().await?;
            let mut u = st.url.clone();
            u.set_path("settings");
            st.wd.goto(u.as_str()).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-settings.png"))
                .await?;
            let mut u = st.url.clone();
            u.set_path("projects/git-helloworld");
            st.wd.goto(u.as_str()).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-project.png"))
                .await?;
            Ok(())
        }
        Action::Install => {
            // there is nothing to install for redmine
            Ok(())
        }
    }
}

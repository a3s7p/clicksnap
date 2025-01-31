use crate::{Action, State};
use thirtyfour::prelude::*;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            st.wd.goto(st.url.as_str()).await?;

            st.wd
                .screenshot(&st.ssp.join("screenshot-landing.png"))
                .await?;

            let u = st.url.join("user/login")?;
            st.wd.goto(u.as_str()).await?;

            let form = st.wd.form(By::Id("user-login-form")).await?;
            form.set_by_name("name", "admin").await?;
            form.set_by_name("pass", &st.pse.app_pass).await?;

            st.wd
                .screenshot(&st.ssp.join("screenshot-login.png"))
                .await?;

            form.submit().await?;

            st.sleep(1000).await;

            let u = st.url.join("admin/config")?;
            st.wd.goto(u.as_str()).await?;

            st.wd
                .screenshot(&st.ssp.join("screenshot-admin-config.png"))
                .await?;
            Ok(())

        }
        Action::Install => {
            // there is nothing to install
            Ok(())
        }
    }
}

use crate::{Action, State};
use thirtyfour::prelude::*;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            st.wd.goto(st.url.as_str()).await?;

            st.wd
                .screenshot(&st.ssp.join("screenshot-landing.png"))
                .await?;


            let u = st.url.join("Security/login?BackURL=%2Fadmin%2Fpages")?;
            st.wd.goto(u.as_str()).await?;

            let form = st.wd.form(By::Id("MemberLoginForm_LoginForm")).await?;
            form.set_by_name("Email", &st.pse.app_email).await?;
            form.set_by_name("Password", &st.pse.app_pass).await?;

            st.wd
                .screenshot(&st.ssp.join("screenshot-login.png"))
                .await?;

            form.submit().await?;

            st.sleep(1000).await;

            st.wd
                .screenshot(&st.ssp.join("screenshot-logged-in.png"))
                .await?;

            Ok(())

        }
        Action::Install => {
            // there is nothing to install
            Ok(())
        }
    }
}

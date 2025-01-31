use crate::{Action, State};
use thirtyfour::prelude::*;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            // main page
            st.wd.goto(st.url.as_str()).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-landing.png"))
                .await?;
            // about/docs section
            st.wait(By::XPath("//footer"))
                .await?
                .scroll_into_view()
                .await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-about.png"))
                .await?;
            // click user guide link
            st.wait(By::XPath("//a[text() = 'User Guide']"))
                .await?
                .click()
                .await?;
            // switch to the newly created tab! the link is target=_blank
            st.wd
                .switch_to_window(st.wd.windows().await?[1].clone())
                .await?;
            st.sleep(1000).await;
            st.wd
                .screenshot(&st.ssp.join("screenshot-user-guide.png"))
                .await?;
            Ok(())
        }
        Action::Install => {
            // there is nothing to install
            Ok(())
        }
    }
}

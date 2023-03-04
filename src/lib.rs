use js_sys::Promise;
use wasm_bindgen::prelude::*;
use web_sys::*;
use aws_sdk_batch::{Client, Config, DescribeJobsRequest};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let refresh_button = get_element_by_id("refresh-button");
    refresh_button.set_onclick(Some(&Closure::wrap(Box::new(refresh_job_status) as Box<dyn FnMut(_)>) as Rc<RefCell<_>>));

    Ok(())
}

fn refresh_job_status(event: web_sys::MouseEvent) {
    event.prevent_default();

    let job_id_input = get_element_by_id("job-id").dyn_into::<HtmlInputElement>().unwrap();
    let job_id = job_id_input.value();

    console::log_1(&format!("Getting status for job {}", job_id).into());

    let client = Client::new(Config::builder().build());

    let describe_jobs_request = DescribeJobsRequest {
        jobs: Some(vec![job_id]),
        ..Default::default()
    };

    let future = async move {
        let resp = client.describe_jobs(describe_jobs_request).await?;

        let job_status = resp.jobs.unwrap()[0].status.as_ref().unwrap();

        let job_status_str = format!("ジョブ {} の状態: {}", job_id, job_status);

        console::log_1(&job_status_str.into());

        let job_status_element = get_element_by_id("job-status");
        job_status_element.set_inner_text(&job_status_str);

        Ok(())
    };

    wasm_bindgen_futures::spawn_local(future);
}

fn get_element_by_id(id: &str) -> web_sys::Element {
    web_sys::window().unwrap().document().unwrap().get_element_by_id(id).unwrap()
}

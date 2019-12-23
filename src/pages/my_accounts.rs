use chrono::{self, DateTime};
use serde::{Deserialize, Serialize};
use yew::format::json::Json;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};

pub struct AccountsPage {
    link: ComponentLink<Self>,
    state: State,
    fetcher: FetchService,
    callback_fetch_bk_account: Callback<Response<Json<Result<Vec<BkAccount>, failure::Error>>>>,
    callback_change_password: Callback<Response<Json<Result<BkAccount, failure::Error>>>>,
    fetch_task_holder: Option<FetchTask>,
}

#[derive(Debug)]
pub enum Msg {
    PasswordChanged(BkAccount),
    Accounts(Vec<BkAccount>),
    FetchFailed,
    PasswordChangedFailed,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<BkAccount>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BkAccount {
    id: u64,
    username: String,
    userpass: String,
    account_id: u64,
    created_at: DateTime<chrono::Local>,
}

impl Component for AccountsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let state = State { entries: vec![] };
        let mut ap = AccountsPage {
            state,
            callback_fetch_bk_account: link.send_back(
                |response: Response<Json<Result<Vec<BkAccount>, failure::Error>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            Msg::Accounts(body)
                        } else {
                            Msg::PasswordChangedFailed
                        }
                    } else {
                        Msg::PasswordChangedFailed
                    }
                },
            ),
            callback_change_password: link.send_back(
                |response: Response<Json<Result<BkAccount, failure::Error>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            Msg::PasswordChanged(body)
                        } else {
                            Msg::PasswordChangedFailed
                        }
                    } else {
                        Msg::PasswordChangedFailed
                    }
                },
            ),
            fetcher: FetchService::new(),
            fetch_task_holder: None,
            link,
        };
        let post_request = Request::get("")
            .header("Content-Type", "application/json")
            .body(Nothing)
            .expect("Failed to build request.");

        let task = ap
            .fetcher
            .fetch(post_request, ap.callback_fetch_bk_account.clone());
        ap
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchFailed => (),
            _ => panic!("Unexpected response: {:?}", msg),
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
        <div class="content">
        <table class="pure-table pure-table-bordered" style="width:100%;">
            <thead>
                <tr>
                    <th>{"#"}</th>
                    <th>{"用户名"}</th>
                    <th>{"密码"}</th>
                    <th>{"更新密码"}</th>
                </tr>
            </thead>

            <tbody>
            { for self.state.entries.iter().enumerate().map(|a| self.view_account(a)) }


            </tbody>
        </table>
                    </div>
                }
    }
}

impl AccountsPage {
    fn view_account(&self, (idx, account): (usize, &BkAccount)) -> Html<Self> {
        let mut class = "todo".to_string();
        // if entry.editing {
        //     class.push_str(" editing");
        // }
        // if entry.completed {
        //     class.push_str(" completed");
        // }
        html! {

            <tr>
                    <td>{idx}</td>
                    <td>{&account.username}</td>
                    <td>{&account.userpass}</td>
                    <td>{"2009"}</td>
                </tr>

            // <li class=class>
            //     <div class="view">
            //         <input
            //             type="checkbox"
            //             class="toggle"/>
                        // checked=entry.completed/>
                        // onclick=self.link.callback(move |_| Msg::Toggle(idx)) />
                    // <label ondoubleclick=self.link.callback(move |_| Msg::ToggleEdit(idx))>{ &entry.description }</label>
                    // <button class="destroy" onclick=self.link.callback(move |_| Msg::Remove(idx)) />
                // </div>
                // { self.view_entry_edit_input((idx, &entry)) }
            // </li>
        }
    }
}

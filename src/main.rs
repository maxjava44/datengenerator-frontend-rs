use sycamore::{prelude::*};
use wasm_bindgen::prelude::wasm_bindgen;
use sycamore_futures::spawn_local_scoped;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`

    #[wasm_bindgen]
    fn download(blob: Vec<u8>,name: &str);
}

use sycamore_router::{Route, Router, HistoryIntegration, navigate};
#[derive(Route)]
enum AppRoutes {
    #[to("/daten")]
    Daten,
    #[not_found]
    Index,
    #[to("/<region>")]
    Region(String),
    #[to("/land/<land>")]
    Land(String)
}


fn handle_navigate(url : String) {
    navigate(url.as_str())
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            Router(
                integration=HistoryIntegration::new(),
                view= move |cx, route: &ReadSignal<AppRoutes>| {
                    view! { cx,
                        div(class="container") {
                            (
                                match route.get().as_ref() {
                                    AppRoutes::Index => {
                                        let regions : &Signal<Vec<String>> = create_signal(cx,Vec::new());
                                        spawn_local_scoped(cx,async {
                                            let mut region_arr : Vec<String> = Vec::new();
                                            let region_request = reqwest::get(format!("http://{}:8080/namen/regions",option_env!("server").map_or("129.159.203.225",|s| s))).await;
                                            match region_request {
                                                Ok(r) => {
                                                    region_arr = r.json().await.ok().map_or(region_arr,|c| c);
                                                },
                                                Err(_) => {}
                                            }
                                            regions.set(region_arr);
                                        });
                                        view!(cx,
                                            div(class = "d-flex w-full align-items-center justify-content-center"){
                                                Indexed (
                                                    iterable=regions,
                                                    view=|cx, x| {
                                                        let x2 = x.clone();
                                                        view! { cx,
                                                            button(style = "margin:20px",class= "btn btn-primary",on:click= move |_| handle_navigate(x.clone())) {
                                                                (x2)
                                                            }
                                                        }
                                                    },
                                                )
                                                button(style = "margin:20px",class= "btn btn-primary",on:click= move |_| handle_navigate("daten".to_string())) {
                                                    ("Datengenerator")
                                                }
                                            }
                                        )
                                    },
                                    AppRoutes::Daten => {
                                        let female = create_signal(cx,false);
                                        let number = create_signal(cx,0);
                                        let number_string = create_signal(cx,"0".to_string());
                                        let range_vec: &Signal<Vec<i32>> = create_signal(cx,Vec::new());

                                        create_effect(cx, || {
                                            number.set((*number_string.get()).parse().ok().map_or(0,|n| n));
                                        });
                                        create_effect(cx, || {
                                            number.track();
                                            female.track();
                                            //refetch();
                                        });
                                        create_effect(cx, || {
                                            let range = 0..*number.get();
                                            range_vec.set(range.collect());
                                        });

                                        let namen : &Signal<Vec<String>> = create_signal(cx,Vec::new());
                                        let streets : &Signal<Vec<String>> = create_signal(cx,Vec::new());
                                        let emails : &Signal<Vec<String>> = create_signal(cx,Vec::new());
                                        let telnrs : &Signal<Vec<String>> = create_signal(cx,Vec::new());
                                        let datums : &Signal<Vec<String>> = create_signal(cx,Vec::new());

                                        let download = |_| {
                                            let mut output : String = format!("Name{limiter}Adresse{limiter}E-Mail{limiter}Tel Nr.{limiter}Geburtsdatum\n",limiter = ";");
                                            for i in range_vec.get().to_vec() {
                                                output = format!("{}{name}{limiter}{adresse}{limiter}{email}{limiter}{telnr}{limiter}{datum}\n",output,limiter = ";",name = namen.get().to_vec().get(i as usize).unwrap_or(&"".to_string()),adresse = streets.get().to_vec().get(i as usize).unwrap_or(&"".to_string()),email = emails.get().to_vec().get(i as usize).unwrap_or(&"".to_string()),telnr = telnrs.get().to_vec().get(i as usize).unwrap_or(&"".to_string()),datum = datums.get().to_vec().get(i as usize).unwrap_or(&"".to_string()));
                                            }
                                            output = output.replace("ü","ue").replace("ä","ae").replace("ö","oe").replace("ß","ss");
                                            download(Vec::from(output.as_bytes()), "daten.csv");
                                        };

                                        create_effect(cx, move || {
                                            number.track();
                                            female.track();
                                            spawn_local_scoped(cx, async {
                                                let emails_request = reqwest::get(format!("http://{}:8080/namen/email/{}",option_env!("server").map_or("129.159.203.225",|s| s),number_string.get())).await;
                                                match emails_request {
                                                    Ok(r) => {
                                                        emails.set(r.json().await.ok().map_or(emails.get().to_vec(), |result| result));
                                                    },
                                                    Err(_) => {}
                                                }
                                            });
                                            spawn_local_scoped(cx, async {
                                                let streets_request = reqwest::get(format!("http://{}:8080/namen/street/{}",option_env!("server").map_or("129.159.203.225",|s| s),number_string.get())).await;
                                                match streets_request {
                                                    Ok(r) => {
                                                        streets.set(r.json().await.ok().map_or(streets.get().to_vec(), |result| result));
                                                    },
                                                    Err(_) => {}
                                                }
                                            });
                                            spawn_local_scoped(cx, async {
                                                let telnrs_request = reqwest::get(format!("http://{}:8080/namen/telnr/{}",option_env!("server").map_or("129.159.203.225",|s| s),number_string.get())).await;
                                                match telnrs_request {
                                                    Ok(r) => {
                                                        telnrs.set(r.json().await.ok().map_or(telnrs.get().to_vec(), |result| result));
                                                    },
                                                    Err(_) => {}
                                                }
                                            });
                                            spawn_local_scoped(cx, async {
                                                let datums_request = reqwest::get(format!("http://{}:8080/namen/datum/{}",option_env!("server").map_or("129.159.203.225",|s| s),number_string.get())).await;
                                                match datums_request {
                                                    Ok(r) => {
                                                        datums.set(r.json().await.ok().map_or(datums.get().to_vec(), |result| result));
                                                    },
                                                    Err(_) => {}
                                                }
                                            });
                                            spawn_local_scoped(cx, async {
                                                let namen_request = reqwest::get(format!("http://{}:8080/namen/Deutschland/{}/{}",option_env!("server").map_or("129.159.203.225",|s| s),number_string.get(),female.get())).await;
                                                match namen_request {
                                                    Ok(r) => {
                                                        namen.set(r.json().await.ok().map_or(namen.get().to_vec(), |result| result));
                                                    },
                                                    Err(_) => {}
                                                }
                                            });
                                        });

                                        view!(cx,
                                            main {
                                                br {}
                                                form {
                                                    label(for="customRange3",class="form-label") {(format!("Wie viele? ({})",number.get()))}
                                                    input(type="range",class="form-range",min="0",max="50",step="1",value="0",id="customRange3",bind:value = number_string) {}
                                                    div(class = "mb-3 form-switch") {
                                                        input(class="form-check-input", type="checkbox", role="switch", id="flexSwitchCheckDefault",bind:checked = female) {}
                                                        label(class="form-check-label mx-3", for="flexSwitchCheckDefault") {"Weiblich?"}
                                                    }
                                                }
                                                button(class="btn btn-primary",on:click= |_| female.trigger_subscribers()) {"Reload"} br{}
                                                button(class="btn btn-primary mt-1", on:click = download) {"Download as CSV"} br{}
                                                button(class="btn btn-secondary mt-1", on:click= |_| navigate("/")) {"Back to Start"} br{}
                                                br{}

                                                table {
                                                    tbody {
                                                        tr {
                                                            th {"Name"}
                                                            th {"Adresse"}
                                                            th {"E-Mail"}
                                                            th {"Tel Nr."}
                                                            th {"Geburtsdatum"}
                                                        }
                                                        Keyed(
                                                            iterable=range_vec,
                                                            view=move |cx, x| view! { cx,
                                                                tr {
                                                                    td { ({
                                                                        namen.get().to_vec().get(x as usize).unwrap_or(&"".to_string()).clone()
                                                                    }) }
                                                                    td { ({
                                                                        streets.get().to_vec().get(x as usize).unwrap_or(&"".to_string()).clone()
                                                                    }) }
                                                                    td { ({
                                                                        emails.get().to_vec().get(x as usize).unwrap_or(&"".to_string()).clone()
                                                                    }) }
                                                                    td { ({
                                                                        telnrs.get().to_vec().get(x as usize).unwrap_or(&"".to_string()).clone()
                                                                    }) }
                                                                    td { ({
                                                                        datums.get().to_vec().get(x as usize).unwrap_or(&"".to_string()).clone()
                                                                    }) }
                                                                }

                                                            },
                                                            key=|x| *x,
                                                        )
                                                    }
                                                }
                                            }
                                        )
                                    },
                                    AppRoutes::Region(region) => {
                                        let region = region.clone();
                                        let laender : &Signal<Vec<String>> = create_signal(cx,Vec::new());
                                        spawn_local_scoped(cx, async move {
                                            let laender_request = reqwest::get(format!("http://{}:8080/namen/countries/{}",option_env!("server").map_or("129.159.203.225",|s| s),region)).await;
                                            match laender_request {
                                                Ok(r) => {
                                                    laender.set(r.json().await.ok().unwrap_or(laender.get().to_vec()))
                                                },
                                                Err(_) => handle_navigate(format!("/{}",region))
                                            }
                                        });
                                        view!(cx,
                                            main {
                                                div(class= "container text-center") {
                                                    Indexed(
                                                        iterable=laender,
                                                        view= |cx, x| view! { cx,
                                                            ({
                                                                let x_clone = x.clone();
                                                                view! {cx,
                                                                    button(class="btn btn-primary m-1", on:click = move |_| handle_navigate(format!("/land/{}",x_clone))) {(x)}
                                                                }
                                                            })

                                                        }
                                                    )
                                                }
                                            }

                                        )
                                    },
                                    AppRoutes::Land(land) => {
                                        let land = land.clone();
                                        let female = create_signal(cx,false);
                                        let number = create_signal(cx,"0".to_string());

                                        let namen : &Signal<Vec<String>> = create_signal(cx,Vec::new());
                                        create_effect(cx, move || {
                                            let land = land.clone();
                                            female.track();
                                            number.track();
                                            spawn_local_scoped(cx, async move{
                                                let server = option_env!("server").map_or("129.159.203.225",|s| s);
                                                let namen_request = reqwest::get(format!("http://{}:8080/namen/{}/{}/{}",server,land,number.get(),female.get())).await;
                                                match namen_request {
                                                    Ok(r) => namen.set(r.json().await.ok().unwrap_or(namen.get().to_vec())),
                                                    Err(_) => {}
                                                }
                                            });
                                        });
                                        view!(cx,
                                            main {
                                                div {
                                                    br{}

                                                    form{
                                                        label (for="customRange3",class="form-label") {(format!("Wie viele ({})",number.get()))}
                                                        input (type="range",class="form-range",min="0",max="50",step="1",value="0",id="customRange3",bind:value = number){}
                                                        div (class="mb-3 form-switch") {
                                                            input (class="form-check-input",type="checkbox",role="switch",id="flexSwitchCheckDefault",bind:checked=female) {}
                                                            label (class="form-check-label mx-3", for="flexSwitchCheckDefault") {"Weiblich?"}
                                                        }
                                                    }

                                                    button (class="btn btn-primary", on:click = |_| female.trigger_subscribers()) {"Reload"} br{}
                                                    button (class="btn btn-primary mt-1", on:click = |_| {
                                                        let mut output = format!("Vorname{limiter}Nachname\n",limiter = ";");

                                                        for name in namen.get_untracked().to_vec() {
                                                            let name = name.replace("ü","ue").replace("ä","ae").replace("ö","oe").replace("ß","ss");
                                                            let split : Vec<&str> = name.split(" ").collect();
                                                            let vorname = split.get(0).unwrap_or(&"");
                                                            let nachname = split.get(1).unwrap_or(&"");

                                                            output = format!("{}{vorname}{limiter}{nachname}\n",output,limiter = ";");
                                                        }
                                                        download(Vec::from(output.as_bytes()), "data.csv");
                                                    }){"Download as CSV"}br{}

                                                    button( type="button", class="btn btn-secondary mt-1", on:click= |_| navigate("/")) {"Back to Start"}

                                                    br{}
                                                    br{}

                                                    Indexed(
                                                        iterable=namen,
                                                        view=|cx, x| view! { cx,
                                                            li {(x)}
                                                        }
                                                    )


                                                }
                                            }
                                        )
                                    },
                                }
                            )
                        }
                    }
                }
            )
        }
    });
}

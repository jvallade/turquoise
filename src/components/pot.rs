use gloo_console::log;
use yew::{function_component, html, Callback, Component, Context, Html, Properties};

use crate::components::tile::{Tile, TileColor};

use super::tile::TileState;

#[derive(PartialEq, Properties)]
pub struct PotAreaProps {
    pub pot_area_update: Callback<(String, TileColor)>,
    pub pots: Vec<PotState>,
}

#[function_component]
pub fn PotArea(props: &PotAreaProps) -> Html {
    let PotAreaProps {
        pot_area_update,
        pots,
    } = props;
    log!(format!("{:?}", pots));
    html! {
        <div class="p-big bg-info">
            <div class="row">
                {
                    for pots.iter()
                        .filter(|pot| matches!(pot.pot_type, PotType::Pot(_)))
                        .map(|pot| html!(<Pot tiles={pot.tiles.clone()}/>))
                }
            </div>
            // <div class="row">
            //     <CommonPot tiles={pots_state.common_pot.clone()}/>
            // </div>
        </div>
    }
}

#[derive(PartialEq, Debug)]
pub struct PotState {
    pub pot_type: PotType,
    pub tiles: Vec<TileState>,
}

#[derive(PartialEq, Debug)]
pub enum PotType {
    Pot(u8),
    Common,
}

pub struct CommonPot;

#[derive(Properties, PartialEq)]
pub struct CommonPotProps {
    tiles: Vec<TileColor>,
}

impl Component for CommonPot {
    type Message = PotMsg;
    type Properties = CommonPotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PotMsg::TileClicked => {
                log!("Tile clicked (from common pot)")
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CommonPotProps { tiles } = ctx.props();

        // let tile_clicked = ctx.link().callback(|_| PotMsg::TileClicked);
        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        // {
                        //     for tiles.iter().map(|c| html!{
                        //         <Tile
                        //             click_handler={tile_clicked}
                        //             color={(*c).clone()}
                        //         />
                        //     })
                        // }
                    </div>
                </div>
            </div>
        }
    }
}

pub struct Pot;

#[derive(Properties, PartialEq)]
pub struct PotProps {
    tiles: Vec<TileState>,
}

pub enum PotMsg {
    TileClicked,
}

impl Component for Pot {
    type Message = PotMsg;
    type Properties = PotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PotProps { tiles } = ctx.props();

        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        {
                            for tiles.iter().map(|s| {
                                let tile_clicked = ctx.link().callback(|_| PotMsg::TileClicked);

                                html!{
                                    <Tile
                                    click_handler={tile_clicked}
                                    color={s.color.clone()}
                                    id={s.id}
                                    />
                                }
                            })
                        }
                    </div>
                </div>
            </div>
        }
    }
}

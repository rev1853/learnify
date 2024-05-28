#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Order, Response, StdError, to_binary};
use crate::state::citizen_info::CitizenInfo;
use crate::state::creator_info::CreatorInfo;
use crate::state::song_info::SongInfo;
use crate::state::state::State;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateParams
) -> Result<Response, StdError> {
    let creator_info = CreatorInfo {
        age: msg.age,
        email: msg.email,
        name: msg.name,
        phone: msg.phone
    };

    let state = State::new();
    state.creator_info.save(deps.storage, &creator_info)?;

    return Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateParams
) -> Result<Response, StdError> {
    let state = State::new();

    // seeder
    let song1 = SongInfo {
        id: "1".to_string(),
        title: "if i had a gun".to_string(),
        artist: "Noel Gallagher".to_string(),
        genre: "Pop".to_string(),
        album: "High flying birds".to_string(),
    };

    let song2 = SongInfo {
        id: "2".to_string(),
        title: "the death of you and me".to_string(),
        artist: "Noel Gallagher".to_string(),
        genre: "Pop".to_string(),
        album: "High flying birds".to_string(),
    };

    let song3 = SongInfo {
        id: "3".to_string(),
        title: "cundamani".to_string(),
        artist: "Denny Caknan".to_string(),
        genre: "Dangdut".to_string(),
        album: "unknown".to_string(),
    };

    state.songs.save(deps.storage, &song1.id.clone(), &song1)?;
    state.songs.save(deps.storage, &song2.id.clone(), &song2)?;
    state.songs.save(deps.storage, &song3.id.clone(), &song3)?;

    // menambah data baru increment
    let last_id = state.songs.keys(deps.storage, None, None, Order::Descending)
        .filter_map(|el| el.ok())
        .map(|el| el.parse::<u32>().unwrap())
        .max();

    let id = match last_id {
        Some(last_id) => last_id + 1,
        None => 1
    };

    let song4 = SongInfo {
        id: id.to_string(),
        title: "celengan rindu".to_string(),
        artist: "fiersa besari".to_string(),
        genre: "pop".to_string(),
        album: "merindu".to_string(),
    };

    state.songs.save(deps.storage, &id.to_string(), &song4)?;

    return Ok(Response::new())
}

#[cw_serde]
pub struct MigrateParams {
}

#[cw_serde]
pub struct InstantiateParams {
    pub name: String,
    pub email: String,
    pub age: u32,
    pub phone: String
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query (
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> Result<Binary, StdError> {
    let state = State::new();
    return match msg {
        QueryMsg::Default(_) => {
            let data = state.creator_info.load(deps.storage)?;
            let response = QueryResponse {
                nama: data.name,
                surel: data.email,
                umur: data.age,
                telepon: data.phone
            };

            Ok(to_binary(&response)?)
        },
        QueryMsg::AllCitizens(_) => {
            let citizens = state.citizens.range(deps.storage, None, None, Order::Ascending)
                .filter_map(|el| { return el.ok(); })
                .map(|el| { return el.1; })
                .collect::<Vec<CitizenInfo>>();

            return Ok(to_binary(&AllCitizenResponse {
                citizens
            })?);
        },
        QueryMsg::Citizen { nik } => {
            let citizen = state.citizens.load(deps.storage, nik)?;
            return Ok(to_binary(&CitizenResponse {
                citizen
            })?)
        },
        QueryMsg::SongByArtist { artist } => {
            let songs = state.songs.idx.artist.prefix(artist)
                .range(deps.storage, None, None, Order::Ascending)
                .filter_map(|el| el.ok())
                .map(|el| el.1)
                .collect::<Vec<SongInfo>>();

            let response = SongsResponse {
                songs
            };

            return Ok(to_binary(&response)?)
        }
    }
}

#[cw_serde]
pub enum QueryMsg {
    Default (Empty),
    AllCitizens (Empty),
    Citizen {
        nik: String
    },
    SongByArtist {
        artist: String
    }
}

#[cw_serde]
pub struct QueryResponse {
    pub nama: String,
    pub surel: String,
    pub umur: u32,
    pub telepon: String
}

#[cw_serde]
pub struct AllCitizenResponse {
    pub citizens: Vec<CitizenInfo>
}

#[cw_serde]
pub struct SongsResponse {
    pub songs: Vec<SongInfo>
}

#[cw_serde]
pub struct CitizenResponse {
    pub citizen: CitizenInfo
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    let state = State::new();
    return match msg {
        ExecuteMsg::UpdateCreator { name, age, email, phone } => {
            let creator_info = CreatorInfo {
                age: age.clone(),
                name: name.clone(),
                email: email.clone(),
                phone: phone.clone()
            };
            state.creator_info.update(deps.storage, |_el| {
                return Ok::<CreatorInfo, StdError>(creator_info);
            })?;

            let res = Response::new()
                .add_attribute("age", age.to_string())
                .add_attribute("name", name)
                .add_attribute("email", email)
                .add_attribute("result", "update sukses!!!");

            Ok(res)
        },
        ExecuteMsg::CreateCitizen (msg) => {
            state.citizens.save(deps.storage, msg.nik, &msg.citizen.clone())?;
            return Ok(Response::new())
        },
        ExecuteMsg::UpdateCitizen (msg) => {
            state.citizens.update(deps.storage, msg.nik, |el| {
                return match el {
                    None => Err(StdError::generic_err("nik tidak valid")),
                    Some(_) => Ok(msg.citizen.clone())
                }
            })?;
            return Ok(Response::new())
        },
        ExecuteMsg::DeleteCitizen { nik } => {
            state.citizens.remove(deps.storage, nik);
            return Ok(Response::new())
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateCreator {
        name: String,
        email: String,
        age: u32,
        phone: String
    },
    CreateCitizen (CreateCitizenParams),
    UpdateCitizen (UpdateCitizenParams),
    DeleteCitizen {
        nik: String
    }
}

#[cw_serde]
pub struct CreateCitizenParams {
    nik: String,
    citizen: CitizenInfo
}

#[cw_serde]
pub struct UpdateCitizenParams {
    nik: String,
    citizen: CitizenInfo
}
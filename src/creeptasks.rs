use log::{info, warn};
use num_derive::*;
use screeps::*;
use std::cmp::Ordering;

#[derive(Copy, Clone, FromPrimitive)]
pub enum Task {
    Chill = 0,
    Conquer = 1,
    Harvest = 2,
    Build = 3,
}
impl Task {
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

pub fn start_working(creep: screeps::objects::Creep) {
    let activity = match creep.memory().i32("activity") {
        Err(_e) => {
            info!("some error occured, creep will be chilling!!");
            creep.memory().set("activity", Task::Chill.to_int());
            Task::Chill
        }

        Ok(Some(numb)) => match num::FromPrimitive::from_i32(numb) {
            Some(task) => task,
            _ => Task::Chill,
        },
        _ => Task::Chill,
    };

    match activity {
        Task::Conquer => gather_resources_to(conquer_controller, creep),

        Task::Harvest => gather_resources_to(store_resources, creep),

        Task::Build => gather_resources_to(build_things, creep),

        _ => (),
    }
}

fn gather_resources_to(next_action: fn(screeps::objects::Creep), creep: screeps::objects::Creep) {
    if creep.memory().bool("gathering") {
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("gathering", false);
        }
    } else {
        if creep.store_used_capacity(None) == 0 {
            creep.memory().set("gathering", true);
        }
    }

    if creep.memory().bool("gathering") {
        let source = &creep
            .room()
            .expect("room is not visible to you")
            .find(find::SOURCES)[0];

        if creep.pos().is_near_to(source) {
            let r = creep.harvest(source);

            if r != ReturnCode::Ok {
                warn!("couldn't harvest: {:?}", r);
            }
        } else {
            creep.move_to(source);
        }
    } else {
        next_action(creep);
    }
}

fn conquer_controller(creep: screeps::objects::Creep) {
    if let Some(controller) = creep
        .room()
        .expect("room is not visible to you")
        .controller()
    {
        match creep.upgrade_controller(&controller) {
            ReturnCode::NotInRange => {
                creep.move_to(&controller);
                ()
            }
            ReturnCode::Ok => (),
            return_code => warn!("couldn't upgrade: {:?}", return_code),
        }
    } else {
        warn!("creep room has no controller");
    }
}

fn store_resources(creep: screeps::objects::Creep) {
    //    use screeps::constants::find;
    //    use screeps::constants::StructureType;

    let structs = creep
        .room()
        .expect("room is not visible to you")
        .find(find::STRUCTURES);

    let struct_to_store = structs
        .iter()
        .filter(|stroge| match stroge {
            Structure::Extension(st) => st.store_free_capacity(Some(ResourceType::Energy)) > 0,
            Structure::Spawn(st) => st.store_free_capacity(Some(ResourceType::Energy)) > 0,
            Structure::Container(st) => st.store_free_capacity(Some(ResourceType::Energy)) > 0,
            Structure::Tower(st) => st.store_free_capacity(Some(ResourceType::Energy)) > 0,
            _ => false,
        })
        .min_by(|st1, st2| match (st1, st2) {
            (_, Structure::Extension(_)) => Ordering::Greater,
            (Structure::Extension(_), _) => Ordering::Less,
            (_, Structure::Spawn(_)) => Ordering::Less,
            (_, _) => Ordering::Greater,
        });

    if let Some(st) = struct_to_store {
        match st.as_transferable() {
            Some(transf) => match creep.transfer_all(transf, ResourceType::Energy) {
                ReturnCode::NotInRange => {
                    creep.move_to(st);
                    ()
                }
                ReturnCode::Ok => (),
                return_code => warn!("couldn't store resources: {:?}", return_code),
            },
            None => warn!("struct is not transferable!"),
        }
    }
}

fn build_things(creep: screeps::objects::Creep) {
    let constr_sites = creep
        .room()
        .expect("room is not visible to you")
        .find(find::CONSTRUCTION_SITES);

    //let taget_constr_site = constr//constr_site.iter().min_by(|cs1, cs2| )
    if !constr_sites.is_empty() {
        match creep.build(&constr_sites[0]) {
            ReturnCode::NotInRange => {
                creep.move_to(&constr_sites[0]);
                ()
            }
            ReturnCode::Ok => (),
            return_code => warn!("couldn't build building: {:?}", return_code),
        }
    }
}

//mod serde{
//	use serde::{Deserialize, Deserializer, Serialize, Serializer};

//	impl Serialize for Tasks {
//		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//		where
//			S: Serializer,
//		{
//			match sefl {
//				Harvest => 0,
//				Conquer => 1,
//			}
//		}
//	}
//
//
//}

//use stdweb::js;

//__js_serializable_boilerplate!(Tasks);
//impl stdweb::JsSerialize for Tasks {
//	fn _into_js< 'a >( &'a self ) -> stdweb::private::SerializedValue< 'a >{
//		stdweb::private::SerializedUntaggedI32 {
//			value: *self as i32
//		}.into()
//	}
//}

//impl TryFrom<

//_js_serializable_boilerplate!( Tasks );

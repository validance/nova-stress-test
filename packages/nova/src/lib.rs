use ::cosmos::Error;
use prost_types::Any;

#[allow(clippy::too_many_arguments)]
pub mod tx;

pub mod cosmos {
    pub mod proto {
        include!("prost/cosmos_proto.rs");
    }

    pub mod base {
        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }
    }
}

pub mod google {
    pub mod api {
        include!("prost/google.api.rs");
    }
}

pub mod nova {
    pub mod gal {
        pub mod v1 {
            include!("prost/nova.gal.v1.rs");
        }
    }
}

pub trait AnyMsg {
    fn try_to_any(&self, type_url: &str) -> Result<Any, Error>
    where
        Self: prost::Message + Sized,
    {
        let mut buf = Vec::new();
        prost::Message::encode(self, &mut buf).map_err(Error::ProstEncodeError)?;
        Ok(Any {
            type_url: type_url.to_string(),
            value: buf,
        })
    }
}

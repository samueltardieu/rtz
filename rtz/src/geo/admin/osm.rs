//! The [OpenStreetMap](https://www.openstreetmap.org/) admin lookup module.

use std::{collections::HashMap, sync::OnceLock};

use rtz_core::geo::{
    admin::osm::OsmAdmin,
    shared::{ConcreteVec, EncodableIds, RoundLngLat},
};

use crate::{
    geo::shared::{HasItemData, HasLookupData},
    CanPerformGeoLookup,
};

#[cfg(feature = "self-contained")]
use include_bytes_aligned::include_bytes_aligned;

// Trait impls.

impl HasItemData for OsmAdmin {
    fn get_mem_items() -> &'static ConcreteVec<OsmAdmin> {
        static TIMEZONES: OnceLock<ConcreteVec<OsmAdmin>> = OnceLock::new();

        #[cfg(feature = "self-contained")]
        {
            TIMEZONES.get_or_init(|| crate::geo::shared::decode_binary_data(ADMIN_BINCODE))
        }

        #[cfg(not(feature = "self-contained"))]
        {
            use rtz_core::geo::{admin::osm::get_geojson_features_from_source, shared::get_items_from_features};

            TIMEZONES.get_or_init(|| {
                let features = get_geojson_features_from_source();

                get_items_from_features(features)
            })
        }
    }
}

impl HasLookupData for OsmAdmin {
    type Lookup = EncodableIds;

    fn get_mem_lookup() -> &'static HashMap<RoundLngLat, Self::Lookup> {
        static CACHE: OnceLock<HashMap<RoundLngLat, EncodableIds>> = OnceLock::new();

        #[cfg(feature = "self-contained")]
        {
            CACHE.get_or_init(|| crate::geo::shared::decode_binary_data(LOOKUP_BINCODE))
        }

        #[cfg(not(feature = "self-contained"))]
        {
            use rtz_core::geo::shared::get_lookup_from_geometries;

            CACHE.get_or_init(|| {
                let cache = get_lookup_from_geometries(OsmAdmin::get_mem_items());

                cache
            })
        }
    }
}

impl CanPerformGeoLookup for OsmAdmin {}

// Statics.

#[cfg(all(host_family_unix, feature = "self-contained"))]
static ADMIN_BINCODE: &[u8] = include_bytes_aligned!(8, "../../../../assets/osm_admins.bincode");
#[cfg(all(host_family_windows, feature = "self-contained"))]
static ADMIN_BINCODE: &[u8] = include_bytes_aligned!(8, "..\\..\\..\\..\\assets\\osm_admins.bincode");

#[cfg(all(host_family_unix, feature = "self-contained"))]
static LOOKUP_BINCODE: &[u8] = include_bytes_aligned!(8, "../../../../assets/osm_admin_lookup.bincode");
#[cfg(all(host_family_windows, feature = "self-contained"))]
static LOOKUP_BINCODE: &[u8] = include_bytes_aligned!(8, "..\\..\\..\\..\\assets\\osm_admin_lookup.bincode");

// Tests.

#[cfg(test)]
mod tests {
    use crate::geo::shared::{CanPerformGeoLookup, HasItemData, MapIntoItems};

    use super::*;
    use pretty_assertions::assert_eq;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    use rtz_core::base::types::Float;

    #[test]
    fn can_get_timezones() {
        let admins = OsmAdmin::get_mem_items();
        assert_eq!(admins.len(), 318111);
    }

    #[test]
    fn can_get_lookup() {
        let cache = OsmAdmin::get_mem_lookup();
        assert_eq!(cache.len(), 64_800);
    }

    #[test]
    fn can_get_from_lookup() {
        let lookup = OsmAdmin::get_lookup_suggestions(-121, 46).unwrap();
        assert_eq!(lookup.len(), 20);
    }

    #[test]
    fn can_perform_exact_lookup() {
        assert_eq!(OsmAdmin::lookup_slow(-177.0, -15.0).len(), 0);
        assert_eq!(OsmAdmin::lookup_slow(-121.0, 46.0)[0].name.as_ref(), "United States");

        assert_eq!(OsmAdmin::lookup_slow(179.9968, -67.0959).len(), 0);
    }

    #[test]
    fn can_access_lookup() {
        let cache = OsmAdmin::get_mem_lookup();

        let tzs = cache.get(&(-177, -15)).map_into_items().unwrap() as Vec<&OsmAdmin>;
        assert_eq!(tzs.len(), 0);

        let tzs = cache.get(&(-121, 46)).map_into_items().unwrap() as Vec<&OsmAdmin>;
        assert_eq!(tzs.len(), 20);

        let tz = cache.get(&(-121, 46)).map_into_items().unwrap()[0] as &OsmAdmin;
        assert_eq!(tz.name.as_ref(), "United States");

        let tzs = cache.get(&(-87, 38)).map_into_items().unwrap() as Vec<&OsmAdmin>;
        assert_eq!(tzs.len(), 58);
    }

    #[test]
    fn can_verify_lookup_assisted_accuracy() {
        (0..100).into_par_iter().for_each(|_| {
            let x = rand::random::<Float>() * 360.0 - 180.0;
            let y = rand::random::<Float>() * 180.0 - 90.0;
            let full = OsmAdmin::lookup_slow(x, y);
            let lookup_assisted = OsmAdmin::lookup(x, y);

            assert_eq!(
                full.into_iter().map(|t| t.id).collect::<Vec<_>>(),
                lookup_assisted.into_iter().map(|t| t.id).collect::<Vec<_>>(),
                "({}, {})",
                x,
                y
            );
        });
    }
}

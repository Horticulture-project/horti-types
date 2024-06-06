use serde_json::{json, Value};
pub fn get_neighbor_json(idx: usize) -> Value {
    match idx {
        _ => json!({
            "apiVersion": "1.0",
            "data": [
            {
                    "kind": "Neighbor",
                    "rloc16": 1004,
                    "rxOnIdle": true,
                    "child": false,
                    "ftd": true,
                    "fnd": false,
                    "rssi": -43,
                    "mLinkQuality": 2,
                    "mAverageRssi": -55
            },
            {
                    "kind": "Neighbor",
                    "rloc16": 1005,
                    "rxOnIdle": true,
                    "child": false,
                    "ftd": true,
                    "fnd": false,
                    "rssi": -43,
                    "mLinkQuality": 2,
                    "mAverageRssi": -55
            },
            {
                    "kind": "Neighbor",
                    "rloc16": 1006,
                    "rxOnIdle": true,
                    "child": false,
                    "ftd": true,
                    "fnd": false,
                    "rssi": -43,
                    "mLinkQuality": 2,
                    "mAverageRssi": -55
            },
            {
                    "kind": "Neighbor",
                    "rloc16": 1007,
                    "rxOnIdle": true,
                    "child": false,
                    "ftd": true,
                    "fnd": false,
                    "rssi": -43,
                    "mLinkQuality": 2,
                    "mAverageRssi": -55
            },
            {
                    "kind": "Neighbor",
                    "rloc16": 1008,
                    "rxOnIdle": true,
                    "child": false,
                    "ftd": true,
                    "fnd": false,
                    "rssi": -43,
                    "mLinkQuality": 2,
                    "mAverageRssi": -55
            },
            {
                    "kind": "Neighbor",
                    "rloc16": 1009,
                    "rxOnIdle": true,
                    "child": false,
                    "ftd": true,
                    "fnd": false,
                    "rssi": -43,
                    "mLinkQuality": 2,
                    "mAverageRssi": -55
            },
            {
                    "kind": "Neighbor",
                    "rloc16": 1010,
                    "rxOnIdle": true,
                    "child": false,
                    "ftd": true,
                    "fnd": false,
                    "rssi": -43,
                    "mLinkQuality": 2,
                    "mAverageRssi": -55
            }

            ]
        }),
    }
}

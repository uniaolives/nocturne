use nocturne_core::node::{ArkheNode, IonTrapArray, QHttpTransducer, FastMarchingSolver};
use nocturne_core::hardware::vercore::VerCoreRV32I;
use nocturne_core::hardware::kiutra::ContinuousADR;
use nocturne_core::transformer::xi_pi::PiXiTransformer;

#[test]
fn test_arkhe_node_integration() {
    let mut node = ArkheNode {
        vercore: VerCoreRV32I { id: "VERCORE-001".to_string() },
        cryostat: ContinuousADR { base_temp_mk: 300000.0 }, // Room temp
        ion_trap: IonTrapArray,
        qhttp: QHttpTransducer,
        fmm: FastMarchingSolver { location: "PRIME-NODE".to_string() },
        transformer: PiXiTransformer::new(),
    };

    // Genesis sequence
    let node_id = node.genesis().expect("Failed to bootstrap node");
    assert_eq!(node_id, "VERCORE-001");
    assert_eq!(node.cryostat.base_temp_mk, 100.0);

    // Successful injection
    let receipt = node.inject("2140_DATA_PACKET".to_string(), 1000.0)
        .expect("Failed to process injection");
    assert_eq!(receipt.arrival_time, 2026);

    // Aliasing failure (bandwidth > cutoff)
    let result = node.inject("HIGH_BW_DATA".to_string(), 3000.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Error: Aliasing");
}

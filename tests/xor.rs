use ritenn::{Activation, HaltCondition, LearningMode, NN};

#[test]
fn xor_4layers() {
    // create examples of the xor function
    let examples = [
        (vec![0f64, 0f64], vec![0f64]),
        (vec![0f64, 1f64], vec![1f64]),
        (vec![1f64, 0f64], vec![1f64]),
        (vec![1f64, 1f64], vec![0f64]),
    ];

    // create a new neural network
    let mut net1 = NN::new(&[2, 3, 3, 1], Activation::PELU, Activation::Sigmoid);

    // train the network
    net1.train(&examples)
        .log_interval(Some(1000))
        .halt_condition(HaltCondition::MSE(0.01))
        .learning_mode(LearningMode::Incremental)
        .momentum(0.1)
        .go();

    // make sure flexbuffers encoding/decoding works as expected
    #[cfg(feature = "serde_flexbuffers")]
    let flexbuffers = net1.to_flexbuffers();
    #[cfg(feature = "serde_flexbuffers")]
    let net2 = NN::from_flexbuffers(&flexbuffers);

    // test the trained network
    for &(ref inputs, ref outputs) in examples.iter() {
        #[cfg(not(feature = "serde_flexbuffers"))]
        let results = net1.run(inputs);
        #[cfg(feature = "serde_flexbuffers")]
        let results = net2.run(inputs);
        let (result, key) = (results[0].round(), outputs[0]);
        assert!(result == key);
    }
}

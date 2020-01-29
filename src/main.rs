use gstreamer::{self, ElementFactory, GstBinExtManual, GstObjectExt};

fn make_element(element_name: &str, name: &str) -> gstreamer::Element {
    match ElementFactory::make(element_name, Some(name)) {
        Ok(e) => {
            println!("Successfully created element \"{}\"", e.get_name());
            e
        }
        _ => panic!("Failed to crate element \"{}\"", name),
    }
}

fn main() {
    match gstreamer::init() {
        Ok(()) => println!("Successfully initialized gstreamer."),
        _ => panic!("Failed to initialize gstreamer."),
    };

    let (major, minor, micro, nano) = gstreamer::version();

    println!("gstreamer version: {}.{}.{}.{}", major, minor, micro, nano);

    let source_element = make_element("fakesrc", "source");
    let filter_element = make_element("identity", "filter");
    let sink_element = make_element("fakesink", "sink");

    let pipeline = gstreamer::Pipeline::new(Some("pipeline"));

    let element_list = [&source_element, &filter_element, &sink_element];

    match pipeline.add_many(&element_list) {
        Ok(()) => println!("Elements added to pipeline."),
        _ => panic!("Failed to add elements to pipeline."),
    };

    match gstreamer::Element::link_many(&element_list) {
        Ok(()) => println!("Elements linked."),
        _ => panic!("Failed to link elements."),
    };
}

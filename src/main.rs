#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate glob;

use std::path::Path;

mod parser;
mod filter_decoder;
mod locs_decoder;
mod cbcl_header_decoder;
mod extract_reads;

fn main() {
    // let filename_info = Path::new("test_data/RunInfo.xml");

    // let cbcl_file = Path::new("/usr/src/bcl2fastr_testlane/C117.1/L001_1.cbcl");
    // let locs_file = Path::new("test_data/test_locs.locs");
    // let filter_file = Path::new("test_data/test_filter.filter");
    // parser::parse_run_info(filename_info);
    // cbcl_header_decoder::cbcl_header_decoder(cbcl_file);
    // locs_decoder::locs_decoder(locs_file);
    // filter_decoder::filter_decoder(filter_file);

    let locs_path = Path::new("test_data/190414_A00111_0296_AHJCWWDSXX/Data/Intensities/s.locs");
    let run_info_path = Path::new("test_data/190414_A00111_0296_AHJCWWDSXX/RunInfo.xml");
    let lane_path = Path::new("test_data/190414_A00111_0296_AHJCWWDSXX/Data/Intensities/BaseCalls/L001");

    extract_reads::extract_reads(locs_path, run_info_path, lane_path, vec![0]);


}

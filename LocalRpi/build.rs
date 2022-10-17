fn main() {
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("protos")
        .include("../proto")
        .input("../proto/light_energy_menagment_system.proto")
        .run_from_script();
}

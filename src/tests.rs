use crate::fcm_file::FcmFile;

fn test_file(path: &str) -> FcmFile {
    let original = std::fs::read(path).unwrap();
    let parsed = FcmFile::from_bytes(original.as_slice()).unwrap();
    let serialized = parsed.to_bytes().unwrap();
    assert_eq!(serialized, original);
    parsed
}

#[test]
fn parses_brother_project100_part1() {
    test_file("samples/brother/project100_part1.fcm");
}

#[test]
fn parses_brother_project100_part2() {
    test_file("samples/brother/project100_part2.fcm");
}

#[test]
fn parses_brother_project100_part3() {
    test_file("samples/brother/project100_part3.fcm");
}

#[test]
fn parses_brother_project100_part4() {
    test_file("samples/brother/project100_part4.fcm");
}

#[test]
fn parses_brother_project101_part1() {
    test_file("samples/brother/project101_part1.fcm");
}

#[test]
fn parses_brother_project101_part2() {
    test_file("samples/brother/project101_part2.fcm");
}

#[test]
fn parses_brother_project102_part1() {
    test_file("samples/brother/project102_part1.fcm");
}

#[test]
fn parses_brother_project102_part2() {
    test_file("samples/brother/project102_part2.fcm");
}

#[test]
fn parses_brother_project102_part3() {
    test_file("samples/brother/project102_part3.fcm");
}

#[test]
fn parses_brother_project103_part1() {
    test_file("samples/brother/project103_part1.fcm");
}

#[test]
fn parses_brother_project103_part2() {
    test_file("samples/brother/project103_part2.fcm");
}

#[test]
fn parses_brother_project104_part1() {
    test_file("samples/brother/project104_part1.fcm");
}

#[test]
fn parses_brother_project104_part2() {
    test_file("samples/brother/project104_part2.fcm");
}

#[test]
fn parses_brother_project104_part3() {
    test_file("samples/brother/project104_part3.fcm");
}

#[test]
fn parses_brother_project105_part10() {
    test_file("samples/brother/project105_part10.fcm");
}

#[test]
fn parses_brother_project105_part11() {
    test_file("samples/brother/project105_part11.fcm");
}

#[test]
fn parses_brother_project105_part12() {
    test_file("samples/brother/project105_part12.fcm");
}

#[test]
fn parses_brother_project105_part13() {
    test_file("samples/brother/project105_part13.fcm");
}

#[test]
fn parses_brother_project105_part14() {
    test_file("samples/brother/project105_part14.fcm");
}

#[test]
fn parses_brother_project105_part15() {
    test_file("samples/brother/project105_part15.fcm");
}

#[test]
fn parses_brother_project105_part16() {
    test_file("samples/brother/project105_part16.fcm");
}

#[test]
fn parses_brother_project105_part17() {
    test_file("samples/brother/project105_part17.fcm");
}

#[test]
fn parses_brother_project105_part18() {
    test_file("samples/brother/project105_part18.fcm");
}

#[test]
fn parses_brother_project105_part19() {
    test_file("samples/brother/project105_part19.fcm");
}

#[test]
fn parses_brother_project105_part1() {
    test_file("samples/brother/project105_part1.fcm");
}

#[test]
fn parses_brother_project105_part20() {
    test_file("samples/brother/project105_part20.fcm");
}

#[test]
fn parses_brother_project105_part21() {
    test_file("samples/brother/project105_part21.fcm");
}

#[test]
fn parses_brother_project105_part22() {
    test_file("samples/brother/project105_part22.fcm");
}

#[test]
fn parses_brother_project105_part23() {
    test_file("samples/brother/project105_part23.fcm");
}

#[test]
fn parses_brother_project105_part24() {
    test_file("samples/brother/project105_part24.fcm");
}

#[test]
fn parses_brother_project105_part25() {
    test_file("samples/brother/project105_part25.fcm");
}

#[test]
fn parses_brother_project105_part26() {
    test_file("samples/brother/project105_part26.fcm");
}

#[test]
fn parses_brother_project105_part27() {
    test_file("samples/brother/project105_part27.fcm");
}

#[test]
fn parses_brother_project105_part2() {
    test_file("samples/brother/project105_part2.fcm");
}

#[test]
fn parses_brother_project105_part3() {
    test_file("samples/brother/project105_part3.fcm");
}

#[test]
fn parses_brother_project105_part4() {
    test_file("samples/brother/project105_part4.fcm");
}

#[test]
fn parses_brother_project105_part5() {
    test_file("samples/brother/project105_part5.fcm");
}

#[test]
fn parses_brother_project105_part6() {
    test_file("samples/brother/project105_part6.fcm");
}

#[test]
fn parses_brother_project105_part7() {
    test_file("samples/brother/project105_part7.fcm");
}

#[test]
fn parses_brother_project105_part8() {
    test_file("samples/brother/project105_part8.fcm");
}

#[test]
fn parses_brother_project105_part9() {
    test_file("samples/brother/project105_part9.fcm");
}

#[test]
fn parses_brother_project106_part1() {
    test_file("samples/brother/project106_part1.fcm");
}

#[test]
fn parses_brother_project106_part2() {
    test_file("samples/brother/project106_part2.fcm");
}

#[test]
fn parses_brother_project106_part3() {
    test_file("samples/brother/project106_part3.fcm");
}

#[test]
fn parses_brother_project106_part4() {
    test_file("samples/brother/project106_part4.fcm");
}

#[test]
fn parses_brother_project106_part5() {
    test_file("samples/brother/project106_part5.fcm");
}

#[test]
fn parses_brother_project107_part1() {
    test_file("samples/brother/project107_part1.fcm");
}

#[test]
fn parses_brother_project107_part2() {
    test_file("samples/brother/project107_part2.fcm");
}

#[test]
fn parses_brother_project107_part3() {
    test_file("samples/brother/project107_part3.fcm");
}

#[test]
fn parses_brother_project107_part4() {
    test_file("samples/brother/project107_part4.fcm");
}

#[test]
fn parses_brother_project108_part1() {
    test_file("samples/brother/project108_part1.fcm");
}

#[test]
fn parses_brother_project108_part2() {
    test_file("samples/brother/project108_part2.fcm");
}

#[test]
fn parses_brother_project108_part3() {
    test_file("samples/brother/project108_part3.fcm");
}

#[test]
fn parses_brother_project109_part1() {
    test_file("samples/brother/project109_part1.fcm");
}

#[test]
fn parses_brother_project10_part1() {
    test_file("samples/brother/project10_part1.fcm");
}

#[test]
fn parses_brother_project10_part2() {
    test_file("samples/brother/project10_part2.fcm");
}

#[test]
fn parses_brother_project10_part3() {
    test_file("samples/brother/project10_part3.fcm");
}

#[test]
fn parses_brother_project10_part4() {
    test_file("samples/brother/project10_part4.fcm");
}

#[test]
fn parses_brother_project10_part5() {
    test_file("samples/brother/project10_part5.fcm");
}

#[test]
fn parses_brother_project10_part6() {
    test_file("samples/brother/project10_part6.fcm");
}

#[test]
fn parses_brother_project110_part1() {
    test_file("samples/brother/project110_part1.fcm");
}

#[test]
fn parses_brother_project111_part1() {
    test_file("samples/brother/project111_part1.fcm");
}

#[test]
fn parses_brother_project111_part2() {
    test_file("samples/brother/project111_part2.fcm");
}

#[test]
fn parses_brother_project111_part3() {
    test_file("samples/brother/project111_part3.fcm");
}

#[test]
fn parses_brother_project111_part4() {
    test_file("samples/brother/project111_part4.fcm");
}

#[test]
fn parses_brother_project112_part1() {
    test_file("samples/brother/project112_part1.fcm");
}

#[test]
fn parses_brother_project112_part2() {
    test_file("samples/brother/project112_part2.fcm");
}

#[test]
fn parses_brother_project112_part3() {
    test_file("samples/brother/project112_part3.fcm");
}

#[test]
fn parses_brother_project113_part1() {
    test_file("samples/brother/project113_part1.fcm");
}

#[test]
fn parses_brother_project113_part2() {
    test_file("samples/brother/project113_part2.fcm");
}

#[test]
fn parses_brother_project113_part3() {
    test_file("samples/brother/project113_part3.fcm");
}

#[test]
fn parses_brother_project113_part4() {
    test_file("samples/brother/project113_part4.fcm");
}

#[test]
fn parses_brother_project113_part5() {
    test_file("samples/brother/project113_part5.fcm");
}

#[test]
fn parses_brother_project114_part1() {
    test_file("samples/brother/project114_part1.fcm");
}

#[test]
fn parses_brother_project114_part2() {
    test_file("samples/brother/project114_part2.fcm");
}

#[test]
fn parses_brother_project114_part3() {
    test_file("samples/brother/project114_part3.fcm");
}

#[test]
fn parses_brother_project115_part1() {
    test_file("samples/brother/project115_part1.fcm");
}

#[test]
fn parses_brother_project116_part1() {
    test_file("samples/brother/project116_part1.fcm");
}

#[test]
fn parses_brother_project116_part2() {
    test_file("samples/brother/project116_part2.fcm");
}

#[test]
fn parses_brother_project116_part3() {
    test_file("samples/brother/project116_part3.fcm");
}

#[test]
fn parses_brother_project117_part1() {
    test_file("samples/brother/project117_part1.fcm");
}

#[test]
fn parses_brother_project117_part2() {
    test_file("samples/brother/project117_part2.fcm");
}

#[test]
fn parses_brother_project117_part3() {
    test_file("samples/brother/project117_part3.fcm");
}

#[test]
fn parses_brother_project117_part4() {
    test_file("samples/brother/project117_part4.fcm");
}

#[test]
fn parses_brother_project117_part5() {
    test_file("samples/brother/project117_part5.fcm");
}

#[test]
fn parses_brother_project117_part6() {
    test_file("samples/brother/project117_part6.fcm");
}

#[test]
fn parses_brother_project118_part1() {
    test_file("samples/brother/project118_part1.fcm");
}

#[test]
fn parses_brother_project118_part2() {
    test_file("samples/brother/project118_part2.fcm");
}

#[test]
fn parses_brother_project118_part3() {
    test_file("samples/brother/project118_part3.fcm");
}

#[test]
fn parses_brother_project118_part4() {
    test_file("samples/brother/project118_part4.fcm");
}

#[test]
fn parses_brother_project119_part1() {
    test_file("samples/brother/project119_part1.fcm");
}

#[test]
fn parses_brother_project11_part1() {
    test_file("samples/brother/project11_part1.fcm");
}

#[test]
fn parses_brother_project11_part2() {
    test_file("samples/brother/project11_part2.fcm");
}

#[test]
fn parses_brother_project11_part3() {
    test_file("samples/brother/project11_part3.fcm");
}

#[test]
fn parses_brother_project11_part4() {
    test_file("samples/brother/project11_part4.fcm");
}

#[test]
fn parses_brother_project11_part5() {
    test_file("samples/brother/project11_part5.fcm");
}

#[test]
fn parses_brother_project120_part1() {
    test_file("samples/brother/project120_part1.fcm");
}

#[test]
fn parses_brother_project122_part1() {
    test_file("samples/brother/project122_part1.fcm");
}

#[test]
fn parses_brother_project122_part2() {
    test_file("samples/brother/project122_part2.fcm");
}

#[test]
fn parses_brother_project123_part1() {
    test_file("samples/brother/project123_part1.fcm");
}

#[test]
fn parses_brother_project123_part2() {
    test_file("samples/brother/project123_part2.fcm");
}

#[test]
fn parses_brother_project124_part1() {
    test_file("samples/brother/project124_part1.fcm");
}

#[test]
fn parses_brother_project124_part2() {
    test_file("samples/brother/project124_part2.fcm");
}

#[test]
fn parses_brother_project124_part3() {
    test_file("samples/brother/project124_part3.fcm");
}

#[test]
fn parses_brother_project124_part4() {
    test_file("samples/brother/project124_part4.fcm");
}

#[test]
fn parses_brother_project125_part1() {
    test_file("samples/brother/project125_part1.fcm");
}

#[test]
fn parses_brother_project127_part1() {
    test_file("samples/brother/project127_part1.fcm");
}

#[test]
fn parses_brother_project127_part2() {
    test_file("samples/brother/project127_part2.fcm");
}

#[test]
fn parses_brother_project127_part3() {
    test_file("samples/brother/project127_part3.fcm");
}

#[test]
fn parses_brother_project128_part1() {
    test_file("samples/brother/project128_part1.fcm");
}

#[test]
fn parses_brother_project128_part2() {
    test_file("samples/brother/project128_part2.fcm");
}

#[test]
fn parses_brother_project128_part3() {
    test_file("samples/brother/project128_part3.fcm");
}

#[test]
fn parses_brother_project128_part4() {
    test_file("samples/brother/project128_part4.fcm");
}

#[test]
fn parses_brother_project129_part1() {
    test_file("samples/brother/project129_part1.fcm");
}

#[test]
fn parses_brother_project129_part2() {
    test_file("samples/brother/project129_part2.fcm");
}

#[test]
fn parses_brother_project12_part1() {
    test_file("samples/brother/project12_part1.fcm");
}

#[test]
fn parses_brother_project12_part2() {
    test_file("samples/brother/project12_part2.fcm");
}

#[test]
fn parses_brother_project12_part3() {
    test_file("samples/brother/project12_part3.fcm");
}

#[test]
fn parses_brother_project12_part4() {
    test_file("samples/brother/project12_part4.fcm");
}

#[test]
fn parses_brother_project12_part5() {
    test_file("samples/brother/project12_part5.fcm");
}

#[test]
fn parses_brother_project12_part6() {
    test_file("samples/brother/project12_part6.fcm");
}

#[test]
fn parses_brother_project12_part7() {
    test_file("samples/brother/project12_part7.fcm");
}

#[test]
fn parses_brother_project130_part1() {
    test_file("samples/brother/project130_part1.fcm");
}

#[test]
fn parses_brother_project130_part2() {
    test_file("samples/brother/project130_part2.fcm");
}

#[test]
fn parses_brother_project130_part3() {
    test_file("samples/brother/project130_part3.fcm");
}

#[test]
fn parses_brother_project131_part1() {
    test_file("samples/brother/project131_part1.fcm");
}

#[test]
fn parses_brother_project131_part2() {
    test_file("samples/brother/project131_part2.fcm");
}

#[test]
fn parses_brother_project132_part1() {
    test_file("samples/brother/project132_part1.fcm");
}

#[test]
fn parses_brother_project132_part2() {
    test_file("samples/brother/project132_part2.fcm");
}

#[test]
fn parses_brother_project133_part1() {
    test_file("samples/brother/project133_part1.fcm");
}

#[test]
fn parses_brother_project133_part2() {
    test_file("samples/brother/project133_part2.fcm");
}

#[test]
fn parses_brother_project133_part3() {
    test_file("samples/brother/project133_part3.fcm");
}

#[test]
fn parses_brother_project134_part1() {
    test_file("samples/brother/project134_part1.fcm");
}

#[test]
fn parses_brother_project134_part2() {
    test_file("samples/brother/project134_part2.fcm");
}

#[test]
fn parses_brother_project134_part3() {
    test_file("samples/brother/project134_part3.fcm");
}

#[test]
fn parses_brother_project134_part4() {
    test_file("samples/brother/project134_part4.fcm");
}

#[test]
fn parses_brother_project135_part1() {
    test_file("samples/brother/project135_part1.fcm");
}

#[test]
fn parses_brother_project135_part2() {
    test_file("samples/brother/project135_part2.fcm");
}

#[test]
fn parses_brother_project135_part3() {
    test_file("samples/brother/project135_part3.fcm");
}

#[test]
fn parses_brother_project136_part1() {
    test_file("samples/brother/project136_part1.fcm");
}

#[test]
fn parses_brother_project136_part2() {
    test_file("samples/brother/project136_part2.fcm");
}

#[test]
fn parses_brother_project136_part3() {
    test_file("samples/brother/project136_part3.fcm");
}

#[test]
fn parses_brother_project137_part1() {
    test_file("samples/brother/project137_part1.fcm");
}

#[test]
fn parses_brother_project137_part2() {
    test_file("samples/brother/project137_part2.fcm");
}

#[test]
fn parses_brother_project137_part3() {
    test_file("samples/brother/project137_part3.fcm");
}

#[test]
fn parses_brother_project137_part4() {
    test_file("samples/brother/project137_part4.fcm");
}

#[test]
fn parses_brother_project138_part1() {
    test_file("samples/brother/project138_part1.fcm");
}

#[test]
fn parses_brother_project138_part2() {
    test_file("samples/brother/project138_part2.fcm");
}

#[test]
fn parses_brother_project139_part1() {
    test_file("samples/brother/project139_part1.fcm");
}

#[test]
fn parses_brother_project139_part2() {
    test_file("samples/brother/project139_part2.fcm");
}

#[test]
fn parses_brother_project13_part1() {
    test_file("samples/brother/project13_part1.fcm");
}

#[test]
fn parses_brother_project13_part2() {
    test_file("samples/brother/project13_part2.fcm");
}

#[test]
fn parses_brother_project13_part3() {
    test_file("samples/brother/project13_part3.fcm");
}

#[test]
fn parses_brother_project13_part4() {
    test_file("samples/brother/project13_part4.fcm");
}

#[test]
fn parses_brother_project13_part5() {
    test_file("samples/brother/project13_part5.fcm");
}

#[test]
fn parses_brother_project13_part6() {
    test_file("samples/brother/project13_part6.fcm");
}

#[test]
fn parses_brother_project13_part7() {
    test_file("samples/brother/project13_part7.fcm");
}

#[test]
fn parses_brother_project13_part8() {
    test_file("samples/brother/project13_part8.fcm");
}

#[test]
fn parses_brother_project140_part1() {
    test_file("samples/brother/project140_part1.fcm");
}

#[test]
fn parses_brother_project140_part2() {
    test_file("samples/brother/project140_part2.fcm");
}

#[test]
fn parses_brother_project140_part3() {
    test_file("samples/brother/project140_part3.fcm");
}

#[test]
fn parses_brother_project141_part1() {
    test_file("samples/brother/project141_part1.fcm");
}

#[test]
fn parses_brother_project141_part2() {
    test_file("samples/brother/project141_part2.fcm");
}

#[test]
fn parses_brother_project141_part3() {
    test_file("samples/brother/project141_part3.fcm");
}

#[test]
fn parses_brother_project141_part4() {
    test_file("samples/brother/project141_part4.fcm");
}

#[test]
fn parses_brother_project141_part5() {
    test_file("samples/brother/project141_part5.fcm");
}

#[test]
fn parses_brother_project142_part1() {
    test_file("samples/brother/project142_part1.fcm");
}

#[test]
fn parses_brother_project142_part2() {
    test_file("samples/brother/project142_part2.fcm");
}

#[test]
fn parses_brother_project142_part3() {
    test_file("samples/brother/project142_part3.fcm");
}

#[test]
fn parses_brother_project143_part1() {
    test_file("samples/brother/project143_part1.fcm");
}

#[test]
fn parses_brother_project143_part2() {
    test_file("samples/brother/project143_part2.fcm");
}

#[test]
fn parses_brother_project144_part1() {
    test_file("samples/brother/project144_part1.fcm");
}

#[test]
fn parses_brother_project144_part2() {
    test_file("samples/brother/project144_part2.fcm");
}

#[test]
fn parses_brother_project144_part3() {
    test_file("samples/brother/project144_part3.fcm");
}

#[test]
fn parses_brother_project144_part4() {
    test_file("samples/brother/project144_part4.fcm");
}

#[test]
fn parses_brother_project145_part1() {
    test_file("samples/brother/project145_part1.fcm");
}

#[test]
fn parses_brother_project146_part1() {
    test_file("samples/brother/project146_part1.fcm");
}

#[test]
fn parses_brother_project146_part2() {
    test_file("samples/brother/project146_part2.fcm");
}

#[test]
fn parses_brother_project147_part1() {
    test_file("samples/brother/project147_part1.fcm");
}

#[test]
fn parses_brother_project147_part2() {
    test_file("samples/brother/project147_part2.fcm");
}

#[test]
fn parses_brother_project147_part3() {
    test_file("samples/brother/project147_part3.fcm");
}

#[test]
fn parses_brother_project148_part1() {
    test_file("samples/brother/project148_part1.fcm");
}

#[test]
fn parses_brother_project148_part2() {
    test_file("samples/brother/project148_part2.fcm");
}

#[test]
fn parses_brother_project149_part1() {
    test_file("samples/brother/project149_part1.fcm");
}

#[test]
fn parses_brother_project149_part2() {
    test_file("samples/brother/project149_part2.fcm");
}

#[test]
fn parses_brother_project14_part1() {
    test_file("samples/brother/project14_part1.fcm");
}

#[test]
fn parses_brother_project14_part2() {
    test_file("samples/brother/project14_part2.fcm");
}

#[test]
fn parses_brother_project14_part3() {
    test_file("samples/brother/project14_part3.fcm");
}

#[test]
fn parses_brother_project14_part4() {
    test_file("samples/brother/project14_part4.fcm");
}

#[test]
fn parses_brother_project14_part5() {
    test_file("samples/brother/project14_part5.fcm");
}

#[test]
fn parses_brother_project14_part6() {
    test_file("samples/brother/project14_part6.fcm");
}

#[test]
fn parses_brother_project14_part7() {
    test_file("samples/brother/project14_part7.fcm");
}

#[test]
fn parses_brother_project150_part1() {
    test_file("samples/brother/project150_part1.fcm");
}

#[test]
fn parses_brother_project150_part2() {
    test_file("samples/brother/project150_part2.fcm");
}

#[test]
fn parses_brother_project150_part3() {
    test_file("samples/brother/project150_part3.fcm");
}

#[test]
fn parses_brother_project151_part1() {
    test_file("samples/brother/project151_part1.fcm");
}

#[test]
fn parses_brother_project151_part2() {
    test_file("samples/brother/project151_part2.fcm");
}

#[test]
fn parses_brother_project152_part1() {
    test_file("samples/brother/project152_part1.fcm");
}

#[test]
fn parses_brother_project152_part2() {
    test_file("samples/brother/project152_part2.fcm");
}

#[test]
fn parses_brother_project153_part1() {
    test_file("samples/brother/project153_part1.fcm");
}

#[test]
fn parses_brother_project153_part2() {
    test_file("samples/brother/project153_part2.fcm");
}

#[test]
fn parses_brother_project153_part3() {
    test_file("samples/brother/project153_part3.fcm");
}

#[test]
fn parses_brother_project154_part1() {
    test_file("samples/brother/project154_part1.fcm");
}

#[test]
fn parses_brother_project155_part1() {
    test_file("samples/brother/project155_part1.fcm");
}

#[test]
fn parses_brother_project155_part2() {
    test_file("samples/brother/project155_part2.fcm");
}

#[test]
fn parses_brother_project156_part1() {
    test_file("samples/brother/project156_part1.fcm");
}

#[test]
fn parses_brother_project156_part2() {
    test_file("samples/brother/project156_part2.fcm");
}

#[test]
fn parses_brother_project157_part1() {
    test_file("samples/brother/project157_part1.fcm");
}

#[test]
fn parses_brother_project157_part2() {
    test_file("samples/brother/project157_part2.fcm");
}

#[test]
fn parses_brother_project157_part3() {
    test_file("samples/brother/project157_part3.fcm");
}

#[test]
fn parses_brother_project157_part4() {
    test_file("samples/brother/project157_part4.fcm");
}

#[test]
fn parses_brother_project158_part1() {
    test_file("samples/brother/project158_part1.fcm");
}

#[test]
fn parses_brother_project158_part2() {
    test_file("samples/brother/project158_part2.fcm");
}

#[test]
fn parses_brother_project158_part3() {
    test_file("samples/brother/project158_part3.fcm");
}

#[test]
fn parses_brother_project159_part1() {
    test_file("samples/brother/project159_part1.fcm");
}

#[test]
fn parses_brother_project159_part2() {
    test_file("samples/brother/project159_part2.fcm");
}

#[test]
fn parses_brother_project159_part3() {
    test_file("samples/brother/project159_part3.fcm");
}

#[test]
fn parses_brother_project159_part4() {
    test_file("samples/brother/project159_part4.fcm");
}

#[test]
fn parses_brother_project159_part5() {
    test_file("samples/brother/project159_part5.fcm");
}

#[test]
fn parses_brother_project159_part6() {
    test_file("samples/brother/project159_part6.fcm");
}

#[test]
fn parses_brother_project15_part1() {
    test_file("samples/brother/project15_part1.fcm");
}

#[test]
fn parses_brother_project15_part2() {
    test_file("samples/brother/project15_part2.fcm");
}

#[test]
fn parses_brother_project15_part3() {
    test_file("samples/brother/project15_part3.fcm");
}

#[test]
fn parses_brother_project15_part4() {
    test_file("samples/brother/project15_part4.fcm");
}

#[test]
fn parses_brother_project15_part5() {
    test_file("samples/brother/project15_part5.fcm");
}

#[test]
fn parses_brother_project15_part6() {
    test_file("samples/brother/project15_part6.fcm");
}

#[test]
fn parses_brother_project15_part7() {
    test_file("samples/brother/project15_part7.fcm");
}

#[test]
fn parses_brother_project15_part8() {
    test_file("samples/brother/project15_part8.fcm");
}

#[test]
fn parses_brother_project160_part1() {
    test_file("samples/brother/project160_part1.fcm");
}

#[test]
fn parses_brother_project160_part2() {
    test_file("samples/brother/project160_part2.fcm");
}

#[test]
fn parses_brother_project160_part3() {
    test_file("samples/brother/project160_part3.fcm");
}

#[test]
fn parses_brother_project161_part1() {
    test_file("samples/brother/project161_part1.fcm");
}

#[test]
fn parses_brother_project162_part1() {
    test_file("samples/brother/project162_part1.fcm");
}

#[test]
fn parses_brother_project162_part2() {
    test_file("samples/brother/project162_part2.fcm");
}

#[test]
fn parses_brother_project163_part1() {
    test_file("samples/brother/project163_part1.fcm");
}

#[test]
fn parses_brother_project163_part2() {
    test_file("samples/brother/project163_part2.fcm");
}

#[test]
fn parses_brother_project163_part3() {
    test_file("samples/brother/project163_part3.fcm");
}

#[test]
fn parses_brother_project163_part4() {
    test_file("samples/brother/project163_part4.fcm");
}

#[test]
fn parses_brother_project164_part1() {
    test_file("samples/brother/project164_part1.fcm");
}

#[test]
fn parses_brother_project164_part2() {
    test_file("samples/brother/project164_part2.fcm");
}

#[test]
fn parses_brother_project164_part3() {
    test_file("samples/brother/project164_part3.fcm");
}

#[test]
fn parses_brother_project165_part1() {
    test_file("samples/brother/project165_part1.fcm");
}

#[test]
fn parses_brother_project165_part2() {
    test_file("samples/brother/project165_part2.fcm");
}

#[test]
fn parses_brother_project165_part3() {
    test_file("samples/brother/project165_part3.fcm");
}

#[test]
fn parses_brother_project166_part1() {
    test_file("samples/brother/project166_part1.fcm");
}

#[test]
fn parses_brother_project166_part2() {
    test_file("samples/brother/project166_part2.fcm");
}

#[test]
fn parses_brother_project166_part3() {
    test_file("samples/brother/project166_part3.fcm");
}

#[test]
fn parses_brother_project167_part1() {
    test_file("samples/brother/project167_part1.fcm");
}

#[test]
fn parses_brother_project168_part1() {
    test_file("samples/brother/project168_part1.fcm");
}

#[test]
fn parses_brother_project168_part2() {
    test_file("samples/brother/project168_part2.fcm");
}

#[test]
fn parses_brother_project169_part1() {
    test_file("samples/brother/project169_part1.fcm");
}

#[test]
fn parses_brother_project169_part2() {
    test_file("samples/brother/project169_part2.fcm");
}

#[test]
fn parses_brother_project16_part1() {
    test_file("samples/brother/project16_part1.fcm");
}

#[test]
fn parses_brother_project16_part2() {
    test_file("samples/brother/project16_part2.fcm");
}

#[test]
fn parses_brother_project16_part3() {
    test_file("samples/brother/project16_part3.fcm");
}

#[test]
fn parses_brother_project16_part4() {
    test_file("samples/brother/project16_part4.fcm");
}

#[test]
fn parses_brother_project16_part5() {
    test_file("samples/brother/project16_part5.fcm");
}

#[test]
fn parses_brother_project16_part6() {
    test_file("samples/brother/project16_part6.fcm");
}

#[test]
fn parses_brother_project16_part7() {
    test_file("samples/brother/project16_part7.fcm");
}

#[test]
fn parses_brother_project16_part8() {
    test_file("samples/brother/project16_part8.fcm");
}

#[test]
fn parses_brother_project16_part9() {
    test_file("samples/brother/project16_part9.fcm");
}

#[test]
fn parses_brother_project170_part1() {
    test_file("samples/brother/project170_part1.fcm");
}

#[test]
fn parses_brother_project170_part2() {
    test_file("samples/brother/project170_part2.fcm");
}

#[test]
fn parses_brother_project171_part1() {
    test_file("samples/brother/project171_part1.fcm");
}

#[test]
fn parses_brother_project172_part1() {
    test_file("samples/brother/project172_part1.fcm");
}

#[test]
fn parses_brother_project172_part2() {
    test_file("samples/brother/project172_part2.fcm");
}

#[test]
fn parses_brother_project173_part1() {
    test_file("samples/brother/project173_part1.fcm");
}

#[test]
fn parses_brother_project173_part2() {
    test_file("samples/brother/project173_part2.fcm");
}

#[test]
fn parses_brother_project174_part1() {
    test_file("samples/brother/project174_part1.fcm");
}

#[test]
fn parses_brother_project174_part2() {
    test_file("samples/brother/project174_part2.fcm");
}

#[test]
fn parses_brother_project175_part1() {
    test_file("samples/brother/project175_part1.fcm");
}

#[test]
fn parses_brother_project176_part1() {
    test_file("samples/brother/project176_part1.fcm");
}

#[test]
fn parses_brother_project176_part2() {
    test_file("samples/brother/project176_part2.fcm");
}

#[test]
fn parses_brother_project176_part3() {
    test_file("samples/brother/project176_part3.fcm");
}

#[test]
fn parses_brother_project177_part1() {
    test_file("samples/brother/project177_part1.fcm");
}

#[test]
fn parses_brother_project178_part1() {
    test_file("samples/brother/project178_part1.fcm");
}

#[test]
fn parses_brother_project178_part2() {
    test_file("samples/brother/project178_part2.fcm");
}

#[test]
fn parses_brother_project179_part1() {
    test_file("samples/brother/project179_part1.fcm");
}

#[test]
fn parses_brother_project179_part2() {
    test_file("samples/brother/project179_part2.fcm");
}

#[test]
fn parses_brother_project17_part1() {
    test_file("samples/brother/project17_part1.fcm");
}

#[test]
fn parses_brother_project17_part2() {
    test_file("samples/brother/project17_part2.fcm");
}

#[test]
fn parses_brother_project17_part3() {
    test_file("samples/brother/project17_part3.fcm");
}

#[test]
fn parses_brother_project17_part4() {
    test_file("samples/brother/project17_part4.fcm");
}

#[test]
fn parses_brother_project17_part5() {
    test_file("samples/brother/project17_part5.fcm");
}

#[test]
fn parses_brother_project180_part1() {
    test_file("samples/brother/project180_part1.fcm");
}

#[test]
fn parses_brother_project180_part2() {
    test_file("samples/brother/project180_part2.fcm");
}

#[test]
fn parses_brother_project180_part3() {
    test_file("samples/brother/project180_part3.fcm");
}

#[test]
fn parses_brother_project181_part1() {
    test_file("samples/brother/project181_part1.fcm");
}

#[test]
fn parses_brother_project181_part2() {
    test_file("samples/brother/project181_part2.fcm");
}

#[test]
fn parses_brother_project182_part1() {
    test_file("samples/brother/project182_part1.fcm");
}

#[test]
fn parses_brother_project182_part2() {
    test_file("samples/brother/project182_part2.fcm");
}

#[test]
fn parses_brother_project183_part1() {
    test_file("samples/brother/project183_part1.fcm");
}

#[test]
fn parses_brother_project183_part2() {
    test_file("samples/brother/project183_part2.fcm");
}

#[test]
fn parses_brother_project183_part3() {
    test_file("samples/brother/project183_part3.fcm");
}

#[test]
fn parses_brother_project183_part4() {
    test_file("samples/brother/project183_part4.fcm");
}

#[test]
fn parses_brother_project184_part1() {
    test_file("samples/brother/project184_part1.fcm");
}

#[test]
fn parses_brother_project184_part2() {
    test_file("samples/brother/project184_part2.fcm");
}

#[test]
fn parses_brother_project185_part1() {
    test_file("samples/brother/project185_part1.fcm");
}

#[test]
fn parses_brother_project185_part2() {
    test_file("samples/brother/project185_part2.fcm");
}

#[test]
fn parses_brother_project186_part1() {
    test_file("samples/brother/project186_part1.fcm");
}

#[test]
fn parses_brother_project186_part2() {
    test_file("samples/brother/project186_part2.fcm");
}

#[test]
fn parses_brother_project186_part3() {
    test_file("samples/brother/project186_part3.fcm");
}

#[test]
fn parses_brother_project187_part1() {
    test_file("samples/brother/project187_part1.fcm");
}

#[test]
fn parses_brother_project187_part2() {
    test_file("samples/brother/project187_part2.fcm");
}

#[test]
fn parses_brother_project187_part3() {
    test_file("samples/brother/project187_part3.fcm");
}

#[test]
fn parses_brother_project188_part1() {
    test_file("samples/brother/project188_part1.fcm");
}

#[test]
fn parses_brother_project189_part1() {
    test_file("samples/brother/project189_part1.fcm");
}

#[test]
fn parses_brother_project189_part2() {
    test_file("samples/brother/project189_part2.fcm");
}

#[test]
fn parses_brother_project18_part1() {
    test_file("samples/brother/project18_part1.fcm");
}

#[test]
fn parses_brother_project18_part2() {
    test_file("samples/brother/project18_part2.fcm");
}

#[test]
fn parses_brother_project18_part3() {
    test_file("samples/brother/project18_part3.fcm");
}

#[test]
fn parses_brother_project18_part4() {
    test_file("samples/brother/project18_part4.fcm");
}

#[test]
fn parses_brother_project18_part5() {
    test_file("samples/brother/project18_part5.fcm");
}

#[test]
fn parses_brother_project18_part6() {
    test_file("samples/brother/project18_part6.fcm");
}

#[test]
fn parses_brother_project18_part7() {
    test_file("samples/brother/project18_part7.fcm");
}

#[test]
fn parses_brother_project18_part8() {
    test_file("samples/brother/project18_part8.fcm");
}

#[test]
fn parses_brother_project190_part1() {
    test_file("samples/brother/project190_part1.fcm");
}

#[test]
fn parses_brother_project190_part2() {
    test_file("samples/brother/project190_part2.fcm");
}

#[test]
fn parses_brother_project190_part3() {
    test_file("samples/brother/project190_part3.fcm");
}

#[test]
fn parses_brother_project191_part1() {
    test_file("samples/brother/project191_part1.fcm");
}

#[test]
fn parses_brother_project191_part2() {
    test_file("samples/brother/project191_part2.fcm");
}

#[test]
fn parses_brother_project191_part3() {
    test_file("samples/brother/project191_part3.fcm");
}

#[test]
fn parses_brother_project191_part4() {
    test_file("samples/brother/project191_part4.fcm");
}

#[test]
fn parses_brother_project191_part5() {
    test_file("samples/brother/project191_part5.fcm");
}

#[test]
fn parses_brother_project191_part6() {
    test_file("samples/brother/project191_part6.fcm");
}

#[test]
fn parses_brother_project191_part7() {
    test_file("samples/brother/project191_part7.fcm");
}

#[test]
fn parses_brother_project192_part1() {
    test_file("samples/brother/project192_part1.fcm");
}

#[test]
fn parses_brother_project192_part2() {
    test_file("samples/brother/project192_part2.fcm");
}

#[test]
fn parses_brother_project192_part3() {
    test_file("samples/brother/project192_part3.fcm");
}

#[test]
fn parses_brother_project192_part4() {
    test_file("samples/brother/project192_part4.fcm");
}

#[test]
fn parses_brother_project192_part5() {
    test_file("samples/brother/project192_part5.fcm");
}

#[test]
fn parses_brother_project192_part6() {
    test_file("samples/brother/project192_part6.fcm");
}

#[test]
fn parses_brother_project192_part7() {
    test_file("samples/brother/project192_part7.fcm");
}

#[test]
fn parses_brother_project193_part1() {
    test_file("samples/brother/project193_part1.fcm");
}

#[test]
fn parses_brother_project194_part1() {
    test_file("samples/brother/project194_part1.fcm");
}

#[test]
fn parses_brother_project195_part1() {
    test_file("samples/brother/project195_part1.fcm");
}

#[test]
fn parses_brother_project195_part2() {
    test_file("samples/brother/project195_part2.fcm");
}

#[test]
fn parses_brother_project195_part3() {
    test_file("samples/brother/project195_part3.fcm");
}

#[test]
fn parses_brother_project195_part4() {
    test_file("samples/brother/project195_part4.fcm");
}

#[test]
fn parses_brother_project195_part5() {
    test_file("samples/brother/project195_part5.fcm");
}

#[test]
fn parses_brother_project195_part6() {
    test_file("samples/brother/project195_part6.fcm");
}

#[test]
fn parses_brother_project196_part1() {
    test_file("samples/brother/project196_part1.fcm");
}

#[test]
fn parses_brother_project197_part1() {
    test_file("samples/brother/project197_part1.fcm");
}

#[test]
fn parses_brother_project197_part2() {
    test_file("samples/brother/project197_part2.fcm");
}

#[test]
fn parses_brother_project198_part1() {
    test_file("samples/brother/project198_part1.fcm");
}

#[test]
fn parses_brother_project198_part2() {
    test_file("samples/brother/project198_part2.fcm");
}

#[test]
fn parses_brother_project198_part3() {
    test_file("samples/brother/project198_part3.fcm");
}

#[test]
fn parses_brother_project199_part1() {
    test_file("samples/brother/project199_part1.fcm");
}

#[test]
fn parses_brother_project199_part2() {
    test_file("samples/brother/project199_part2.fcm");
}

#[test]
fn parses_brother_project199_part3() {
    test_file("samples/brother/project199_part3.fcm");
}

#[test]
fn parses_brother_project19_part1() {
    test_file("samples/brother/project19_part1.fcm");
}

#[test]
fn parses_brother_project19_part2() {
    test_file("samples/brother/project19_part2.fcm");
}

#[test]
fn parses_brother_project1_part1() {
    test_file("samples/brother/project1_part1.fcm");
}

#[test]
fn parses_brother_project1_part2() {
    test_file("samples/brother/project1_part2.fcm");
}

#[test]
fn parses_brother_project1_part3() {
    test_file("samples/brother/project1_part3.fcm");
}

#[test]
fn parses_brother_project1_part4() {
    test_file("samples/brother/project1_part4.fcm");
}

#[test]
fn parses_brother_project1_part5() {
    test_file("samples/brother/project1_part5.fcm");
}

#[test]
fn parses_brother_project1_part6() {
    test_file("samples/brother/project1_part6.fcm");
}

#[test]
fn parses_brother_project1_part7() {
    test_file("samples/brother/project1_part7.fcm");
}

#[test]
fn parses_brother_project1_part8() {
    test_file("samples/brother/project1_part8.fcm");
}

#[test]
fn parses_brother_project200_part1() {
    test_file("samples/brother/project200_part1.fcm");
}

#[test]
fn parses_brother_project200_part2() {
    test_file("samples/brother/project200_part2.fcm");
}

#[test]
fn parses_brother_project200_part3() {
    test_file("samples/brother/project200_part3.fcm");
}

#[test]
fn parses_brother_project201_part1() {
    test_file("samples/brother/project201_part1.fcm");
}

#[test]
fn parses_brother_project201_part2() {
    test_file("samples/brother/project201_part2.fcm");
}

#[test]
fn parses_brother_project202_part1() {
    test_file("samples/brother/project202_part1.fcm");
}

#[test]
fn parses_brother_project203_part1() {
    test_file("samples/brother/project203_part1.fcm");
}

#[test]
fn parses_brother_project204_part1() {
    test_file("samples/brother/project204_part1.fcm");
}

#[test]
fn parses_brother_project204_part2() {
    test_file("samples/brother/project204_part2.fcm");
}

#[test]
fn parses_brother_project204_part3() {
    test_file("samples/brother/project204_part3.fcm");
}

#[test]
fn parses_brother_project204_part4() {
    test_file("samples/brother/project204_part4.fcm");
}

#[test]
fn parses_brother_project205_part1() {
    test_file("samples/brother/project205_part1.fcm");
}

#[test]
fn parses_brother_project206_part1() {
    test_file("samples/brother/project206_part1.fcm");
}

#[test]
fn parses_brother_project206_part2() {
    test_file("samples/brother/project206_part2.fcm");
}

#[test]
fn parses_brother_project206_part3() {
    test_file("samples/brother/project206_part3.fcm");
}

#[test]
fn parses_brother_project207_part1() {
    test_file("samples/brother/project207_part1.fcm");
}

#[test]
fn parses_brother_project207_part2() {
    test_file("samples/brother/project207_part2.fcm");
}

#[test]
fn parses_brother_project207_part3() {
    test_file("samples/brother/project207_part3.fcm");
}

#[test]
fn parses_brother_project208_part1() {
    test_file("samples/brother/project208_part1.fcm");
}

#[test]
fn parses_brother_project209_part1() {
    test_file("samples/brother/project209_part1.fcm");
}

#[test]
fn parses_brother_project20_part1() {
    test_file("samples/brother/project20_part1.fcm");
}

#[test]
fn parses_brother_project20_part2() {
    test_file("samples/brother/project20_part2.fcm");
}

#[test]
fn parses_brother_project20_part3() {
    test_file("samples/brother/project20_part3.fcm");
}

#[test]
fn parses_brother_project20_part4() {
    test_file("samples/brother/project20_part4.fcm");
}

#[test]
fn parses_brother_project210_part1() {
    test_file("samples/brother/project210_part1.fcm");
}

#[test]
fn parses_brother_project210_part2() {
    test_file("samples/brother/project210_part2.fcm");
}

#[test]
fn parses_brother_project210_part3() {
    test_file("samples/brother/project210_part3.fcm");
}

#[test]
fn parses_brother_project211_part1() {
    test_file("samples/brother/project211_part1.fcm");
}

#[test]
fn parses_brother_project211_part2() {
    test_file("samples/brother/project211_part2.fcm");
}

#[test]
fn parses_brother_project212_part1() {
    test_file("samples/brother/project212_part1.fcm");
}

#[test]
fn parses_brother_project212_part2() {
    test_file("samples/brother/project212_part2.fcm");
}

#[test]
fn parses_brother_project212_part3() {
    test_file("samples/brother/project212_part3.fcm");
}

#[test]
fn parses_brother_project212_part4() {
    test_file("samples/brother/project212_part4.fcm");
}

#[test]
fn parses_brother_project213_part1() {
    test_file("samples/brother/project213_part1.fcm");
}

#[test]
fn parses_brother_project213_part2() {
    test_file("samples/brother/project213_part2.fcm");
}

#[test]
fn parses_brother_project214_part1() {
    test_file("samples/brother/project214_part1.fcm");
}

#[test]
fn parses_brother_project214_part2() {
    test_file("samples/brother/project214_part2.fcm");
}

#[test]
fn parses_brother_project215_part1() {
    test_file("samples/brother/project215_part1.fcm");
}

#[test]
fn parses_brother_project215_part2() {
    test_file("samples/brother/project215_part2.fcm");
}

#[test]
fn parses_brother_project215_part3() {
    test_file("samples/brother/project215_part3.fcm");
}

#[test]
fn parses_brother_project216_part1() {
    test_file("samples/brother/project216_part1.fcm");
}

#[test]
fn parses_brother_project216_part2() {
    test_file("samples/brother/project216_part2.fcm");
}

#[test]
fn parses_brother_project217_part1() {
    test_file("samples/brother/project217_part1.fcm");
}

#[test]
fn parses_brother_project217_part2() {
    test_file("samples/brother/project217_part2.fcm");
}

#[test]
fn parses_brother_project217_part3() {
    test_file("samples/brother/project217_part3.fcm");
}

#[test]
fn parses_brother_project218_part1() {
    test_file("samples/brother/project218_part1.fcm");
}

#[test]
fn parses_brother_project218_part2() {
    test_file("samples/brother/project218_part2.fcm");
}

#[test]
fn parses_brother_project218_part3() {
    test_file("samples/brother/project218_part3.fcm");
}

#[test]
fn parses_brother_project219_part1() {
    test_file("samples/brother/project219_part1.fcm");
}

#[test]
fn parses_brother_project219_part2() {
    test_file("samples/brother/project219_part2.fcm");
}

#[test]
fn parses_brother_project21_part1() {
    test_file("samples/brother/project21_part1.fcm");
}

#[test]
fn parses_brother_project21_part2() {
    test_file("samples/brother/project21_part2.fcm");
}

#[test]
fn parses_brother_project21_part3() {
    test_file("samples/brother/project21_part3.fcm");
}

#[test]
fn parses_brother_project21_part4() {
    test_file("samples/brother/project21_part4.fcm");
}

#[test]
fn parses_brother_project21_part5() {
    test_file("samples/brother/project21_part5.fcm");
}

#[test]
fn parses_brother_project21_part6() {
    test_file("samples/brother/project21_part6.fcm");
}

#[test]
fn parses_brother_project21_part7() {
    test_file("samples/brother/project21_part7.fcm");
}

#[test]
fn parses_brother_project21_part8() {
    test_file("samples/brother/project21_part8.fcm");
}

#[test]
fn parses_brother_project220_part1() {
    test_file("samples/brother/project220_part1.fcm");
}

#[test]
fn parses_brother_project220_part2() {
    test_file("samples/brother/project220_part2.fcm");
}

#[test]
fn parses_brother_project220_part3() {
    test_file("samples/brother/project220_part3.fcm");
}

#[test]
fn parses_brother_project220_part4() {
    test_file("samples/brother/project220_part4.fcm");
}

#[test]
fn parses_brother_project220_part5() {
    test_file("samples/brother/project220_part5.fcm");
}

#[test]
fn parses_brother_project220_part6() {
    test_file("samples/brother/project220_part6.fcm");
}

#[test]
fn parses_brother_project220_part7() {
    test_file("samples/brother/project220_part7.fcm");
}

#[test]
fn parses_brother_project221_part1() {
    test_file("samples/brother/project221_part1.fcm");
}

#[test]
fn parses_brother_project221_part2() {
    test_file("samples/brother/project221_part2.fcm");
}

#[test]
fn parses_brother_project221_part3() {
    test_file("samples/brother/project221_part3.fcm");
}

#[test]
fn parses_brother_project222_part1() {
    test_file("samples/brother/project222_part1.fcm");
}

#[test]
fn parses_brother_project222_part2() {
    test_file("samples/brother/project222_part2.fcm");
}

#[test]
fn parses_brother_project222_part3() {
    test_file("samples/brother/project222_part3.fcm");
}

#[test]
fn parses_brother_project223_part1() {
    test_file("samples/brother/project223_part1.fcm");
}

#[test]
fn parses_brother_project223_part2() {
    test_file("samples/brother/project223_part2.fcm");
}

#[test]
fn parses_brother_project223_part3() {
    test_file("samples/brother/project223_part3.fcm");
}

#[test]
fn parses_brother_project224_part1() {
    test_file("samples/brother/project224_part1.fcm");
}

#[test]
fn parses_brother_project224_part2() {
    test_file("samples/brother/project224_part2.fcm");
}

#[test]
fn parses_brother_project225_part1() {
    test_file("samples/brother/project225_part1.fcm");
}

#[test]
fn parses_brother_project225_part2() {
    test_file("samples/brother/project225_part2.fcm");
}

#[test]
fn parses_brother_project226_part1() {
    test_file("samples/brother/project226_part1.fcm");
}

#[test]
fn parses_brother_project226_part2() {
    test_file("samples/brother/project226_part2.fcm");
}

#[test]
fn parses_brother_project226_part3() {
    test_file("samples/brother/project226_part3.fcm");
}

#[test]
fn parses_brother_project226_part4() {
    test_file("samples/brother/project226_part4.fcm");
}

#[test]
fn parses_brother_project226_part5() {
    test_file("samples/brother/project226_part5.fcm");
}

#[test]
fn parses_brother_project226_part6() {
    test_file("samples/brother/project226_part6.fcm");
}

#[test]
fn parses_brother_project227_part1() {
    test_file("samples/brother/project227_part1.fcm");
}

#[test]
fn parses_brother_project227_part2() {
    test_file("samples/brother/project227_part2.fcm");
}

#[test]
fn parses_brother_project228_part1() {
    test_file("samples/brother/project228_part1.fcm");
}

#[test]
fn parses_brother_project228_part2() {
    test_file("samples/brother/project228_part2.fcm");
}

#[test]
fn parses_brother_project228_part3() {
    test_file("samples/brother/project228_part3.fcm");
}

#[test]
fn parses_brother_project229_part1() {
    test_file("samples/brother/project229_part1.fcm");
}

#[test]
fn parses_brother_project229_part2() {
    test_file("samples/brother/project229_part2.fcm");
}

#[test]
fn parses_brother_project22_part1() {
    test_file("samples/brother/project22_part1.fcm");
}

#[test]
fn parses_brother_project22_part2() {
    test_file("samples/brother/project22_part2.fcm");
}

#[test]
fn parses_brother_project22_part3() {
    test_file("samples/brother/project22_part3.fcm");
}

#[test]
fn parses_brother_project22_part4() {
    test_file("samples/brother/project22_part4.fcm");
}

#[test]
fn parses_brother_project22_part5() {
    test_file("samples/brother/project22_part5.fcm");
}

#[test]
fn parses_brother_project22_part6() {
    test_file("samples/brother/project22_part6.fcm");
}

#[test]
fn parses_brother_project230_part1() {
    test_file("samples/brother/project230_part1.fcm");
}

#[test]
fn parses_brother_project230_part2() {
    test_file("samples/brother/project230_part2.fcm");
}

#[test]
fn parses_brother_project230_part3() {
    test_file("samples/brother/project230_part3.fcm");
}

#[test]
fn parses_brother_project230_part4() {
    test_file("samples/brother/project230_part4.fcm");
}

#[test]
fn parses_brother_project230_part5() {
    test_file("samples/brother/project230_part5.fcm");
}

#[test]
fn parses_brother_project231_part1() {
    test_file("samples/brother/project231_part1.fcm");
}

#[test]
fn parses_brother_project231_part2() {
    test_file("samples/brother/project231_part2.fcm");
}

#[test]
fn parses_brother_project231_part3() {
    test_file("samples/brother/project231_part3.fcm");
}

#[test]
fn parses_brother_project231_part4() {
    test_file("samples/brother/project231_part4.fcm");
}

#[test]
fn parses_brother_project231_part5() {
    test_file("samples/brother/project231_part5.fcm");
}

#[test]
fn parses_brother_project232_part1() {
    test_file("samples/brother/project232_part1.fcm");
}

#[test]
fn parses_brother_project232_part2() {
    test_file("samples/brother/project232_part2.fcm");
}

#[test]
fn parses_brother_project232_part3() {
    test_file("samples/brother/project232_part3.fcm");
}

#[test]
fn parses_brother_project232_part4() {
    test_file("samples/brother/project232_part4.fcm");
}

#[test]
fn parses_brother_project233_part1() {
    test_file("samples/brother/project233_part1.fcm");
}

#[test]
fn parses_brother_project233_part2() {
    test_file("samples/brother/project233_part2.fcm");
}

#[test]
fn parses_brother_project233_part3() {
    test_file("samples/brother/project233_part3.fcm");
}

#[test]
fn parses_brother_project234_part1() {
    test_file("samples/brother/project234_part1.fcm");
}

#[test]
fn parses_brother_project234_part2() {
    test_file("samples/brother/project234_part2.fcm");
}

#[test]
fn parses_brother_project234_part3() {
    test_file("samples/brother/project234_part3.fcm");
}

#[test]
fn parses_brother_project235_part1() {
    test_file("samples/brother/project235_part1.fcm");
}

#[test]
fn parses_brother_project236_part1() {
    test_file("samples/brother/project236_part1.fcm");
}

#[test]
fn parses_brother_project236_part2() {
    test_file("samples/brother/project236_part2.fcm");
}

#[test]
fn parses_brother_project236_part3() {
    test_file("samples/brother/project236_part3.fcm");
}

#[test]
fn parses_brother_project237_part1() {
    test_file("samples/brother/project237_part1.fcm");
}

#[test]
fn parses_brother_project237_part2() {
    test_file("samples/brother/project237_part2.fcm");
}

#[test]
fn parses_brother_project237_part3() {
    test_file("samples/brother/project237_part3.fcm");
}

#[test]
fn parses_brother_project238_part1() {
    test_file("samples/brother/project238_part1.fcm");
}

#[test]
fn parses_brother_project238_part2() {
    test_file("samples/brother/project238_part2.fcm");
}

#[test]
fn parses_brother_project238_part3() {
    test_file("samples/brother/project238_part3.fcm");
}

#[test]
fn parses_brother_project239_part1() {
    test_file("samples/brother/project239_part1.fcm");
}

#[test]
fn parses_brother_project239_part2() {
    test_file("samples/brother/project239_part2.fcm");
}

#[test]
fn parses_brother_project23_part1() {
    test_file("samples/brother/project23_part1.fcm");
}

#[test]
fn parses_brother_project23_part2() {
    test_file("samples/brother/project23_part2.fcm");
}

#[test]
fn parses_brother_project23_part3() {
    test_file("samples/brother/project23_part3.fcm");
}

#[test]
fn parses_brother_project23_part4() {
    test_file("samples/brother/project23_part4.fcm");
}

#[test]
fn parses_brother_project23_part5() {
    test_file("samples/brother/project23_part5.fcm");
}

#[test]
fn parses_brother_project23_part6() {
    test_file("samples/brother/project23_part6.fcm");
}

#[test]
fn parses_brother_project240_part1() {
    test_file("samples/brother/project240_part1.fcm");
}

#[test]
fn parses_brother_project240_part2() {
    test_file("samples/brother/project240_part2.fcm");
}

#[test]
fn parses_brother_project240_part3() {
    test_file("samples/brother/project240_part3.fcm");
}

#[test]
fn parses_brother_project240_part4() {
    test_file("samples/brother/project240_part4.fcm");
}

#[test]
fn parses_brother_project241_part1() {
    test_file("samples/brother/project241_part1.fcm");
}

#[test]
fn parses_brother_project241_part2() {
    test_file("samples/brother/project241_part2.fcm");
}

#[test]
fn parses_brother_project241_part3() {
    test_file("samples/brother/project241_part3.fcm");
}

#[test]
fn parses_brother_project241_part4() {
    test_file("samples/brother/project241_part4.fcm");
}

#[test]
fn parses_brother_project242_part1() {
    test_file("samples/brother/project242_part1.fcm");
}

#[test]
fn parses_brother_project242_part2() {
    test_file("samples/brother/project242_part2.fcm");
}

#[test]
fn parses_brother_project242_part3() {
    test_file("samples/brother/project242_part3.fcm");
}

#[test]
fn parses_brother_project243_part1() {
    test_file("samples/brother/project243_part1.fcm");
}

#[test]
fn parses_brother_project243_part2() {
    test_file("samples/brother/project243_part2.fcm");
}

#[test]
fn parses_brother_project243_part3() {
    test_file("samples/brother/project243_part3.fcm");
}

#[test]
fn parses_brother_project243_part4() {
    test_file("samples/brother/project243_part4.fcm");
}

#[test]
fn parses_brother_project244_part1() {
    test_file("samples/brother/project244_part1.fcm");
}

#[test]
fn parses_brother_project244_part2() {
    test_file("samples/brother/project244_part2.fcm");
}

#[test]
fn parses_brother_project244_part3() {
    test_file("samples/brother/project244_part3.fcm");
}

#[test]
fn parses_brother_project244_part4() {
    test_file("samples/brother/project244_part4.fcm");
}

#[test]
fn parses_brother_project244_part5() {
    test_file("samples/brother/project244_part5.fcm");
}

#[test]
fn parses_brother_project245_part1() {
    test_file("samples/brother/project245_part1.fcm");
}

#[test]
fn parses_brother_project245_part2() {
    test_file("samples/brother/project245_part2.fcm");
}

#[test]
fn parses_brother_project245_part3() {
    test_file("samples/brother/project245_part3.fcm");
}

#[test]
fn parses_brother_project245_part4() {
    test_file("samples/brother/project245_part4.fcm");
}

#[test]
fn parses_brother_project246_part1() {
    test_file("samples/brother/project246_part1.fcm");
}

#[test]
fn parses_brother_project246_part2() {
    test_file("samples/brother/project246_part2.fcm");
}

#[test]
fn parses_brother_project247_part1() {
    test_file("samples/brother/project247_part1.fcm");
}

#[test]
fn parses_brother_project247_part2() {
    test_file("samples/brother/project247_part2.fcm");
}

#[test]
fn parses_brother_project247_part3() {
    test_file("samples/brother/project247_part3.fcm");
}

#[test]
fn parses_brother_project247_part4() {
    test_file("samples/brother/project247_part4.fcm");
}

#[test]
fn parses_brother_project248_part1() {
    test_file("samples/brother/project248_part1.fcm");
}

#[test]
fn parses_brother_project249_part1() {
    test_file("samples/brother/project249_part1.fcm");
}

#[test]
fn parses_brother_project24_part1() {
    test_file("samples/brother/project24_part1.fcm");
}

#[test]
fn parses_brother_project24_part2() {
    test_file("samples/brother/project24_part2.fcm");
}

#[test]
fn parses_brother_project24_part3() {
    test_file("samples/brother/project24_part3.fcm");
}

#[test]
fn parses_brother_project24_part4() {
    test_file("samples/brother/project24_part4.fcm");
}

#[test]
fn parses_brother_project24_part5() {
    test_file("samples/brother/project24_part5.fcm");
}

#[test]
fn parses_brother_project250_part1() {
    test_file("samples/brother/project250_part1.fcm");
}

#[test]
fn parses_brother_project250_part2() {
    test_file("samples/brother/project250_part2.fcm");
}

#[test]
fn parses_brother_project251_part1() {
    test_file("samples/brother/project251_part1.fcm");
}

#[test]
fn parses_brother_project251_part2() {
    test_file("samples/brother/project251_part2.fcm");
}

#[test]
fn parses_brother_project251_part3() {
    test_file("samples/brother/project251_part3.fcm");
}

#[test]
fn parses_brother_project251_part4() {
    test_file("samples/brother/project251_part4.fcm");
}

#[test]
fn parses_brother_project251_part5() {
    test_file("samples/brother/project251_part5.fcm");
}

#[test]
fn parses_brother_project252_part1() {
    test_file("samples/brother/project252_part1.fcm");
}

#[test]
fn parses_brother_project252_part2() {
    test_file("samples/brother/project252_part2.fcm");
}

#[test]
fn parses_brother_project253_part1() {
    test_file("samples/brother/project253_part1.fcm");
}

#[test]
fn parses_brother_project254_part1() {
    test_file("samples/brother/project254_part1.fcm");
}

#[test]
fn parses_brother_project254_part2() {
    test_file("samples/brother/project254_part2.fcm");
}

#[test]
fn parses_brother_project255_part1() {
    test_file("samples/brother/project255_part1.fcm");
}

#[test]
fn parses_brother_project255_part2() {
    test_file("samples/brother/project255_part2.fcm");
}

#[test]
fn parses_brother_project255_part3() {
    test_file("samples/brother/project255_part3.fcm");
}

#[test]
fn parses_brother_project255_part4() {
    test_file("samples/brother/project255_part4.fcm");
}

#[test]
fn parses_brother_project256_part1() {
    test_file("samples/brother/project256_part1.fcm");
}

#[test]
fn parses_brother_project256_part2() {
    test_file("samples/brother/project256_part2.fcm");
}

#[test]
fn parses_brother_project256_part3() {
    test_file("samples/brother/project256_part3.fcm");
}

#[test]
fn parses_brother_project256_part4() {
    test_file("samples/brother/project256_part4.fcm");
}

#[test]
fn parses_brother_project257_part1() {
    test_file("samples/brother/project257_part1.fcm");
}

#[test]
fn parses_brother_project257_part2() {
    test_file("samples/brother/project257_part2.fcm");
}

#[test]
fn parses_brother_project257_part3() {
    test_file("samples/brother/project257_part3.fcm");
}

#[test]
fn parses_brother_project258_part1() {
    test_file("samples/brother/project258_part1.fcm");
}

#[test]
fn parses_brother_project258_part2() {
    test_file("samples/brother/project258_part2.fcm");
}

#[test]
fn parses_brother_project258_part3() {
    test_file("samples/brother/project258_part3.fcm");
}

#[test]
fn parses_brother_project259_part1() {
    test_file("samples/brother/project259_part1.fcm");
}

#[test]
fn parses_brother_project259_part2() {
    test_file("samples/brother/project259_part2.fcm");
}

#[test]
fn parses_brother_project25_part1() {
    test_file("samples/brother/project25_part1.fcm");
}

#[test]
fn parses_brother_project25_part2() {
    test_file("samples/brother/project25_part2.fcm");
}

#[test]
fn parses_brother_project25_part3() {
    test_file("samples/brother/project25_part3.fcm");
}

#[test]
fn parses_brother_project25_part4() {
    test_file("samples/brother/project25_part4.fcm");
}

#[test]
fn parses_brother_project25_part5() {
    test_file("samples/brother/project25_part5.fcm");
}

#[test]
fn parses_brother_project25_part6() {
    test_file("samples/brother/project25_part6.fcm");
}

#[test]
fn parses_brother_project25_part7() {
    test_file("samples/brother/project25_part7.fcm");
}

#[test]
fn parses_brother_project25_part8() {
    test_file("samples/brother/project25_part8.fcm");
}

#[test]
fn parses_brother_project25_part9() {
    test_file("samples/brother/project25_part9.fcm");
}

#[test]
fn parses_brother_project260_part1() {
    test_file("samples/brother/project260_part1.fcm");
}

#[test]
fn parses_brother_project260_part2() {
    test_file("samples/brother/project260_part2.fcm");
}

#[test]
fn parses_brother_project260_part3() {
    test_file("samples/brother/project260_part3.fcm");
}

#[test]
fn parses_brother_project260_part4() {
    test_file("samples/brother/project260_part4.fcm");
}

#[test]
fn parses_brother_project261_part1() {
    test_file("samples/brother/project261_part1.fcm");
}

#[test]
fn parses_brother_project261_part2() {
    test_file("samples/brother/project261_part2.fcm");
}

#[test]
fn parses_brother_project261_part3() {
    test_file("samples/brother/project261_part3.fcm");
}

#[test]
fn parses_brother_project261_part4() {
    test_file("samples/brother/project261_part4.fcm");
}

#[test]
fn parses_brother_project262_part1() {
    test_file("samples/brother/project262_part1.fcm");
}

#[test]
fn parses_brother_project263_part1() {
    test_file("samples/brother/project263_part1.fcm");
}

#[test]
fn parses_brother_project263_part2() {
    test_file("samples/brother/project263_part2.fcm");
}

#[test]
fn parses_brother_project263_part3() {
    test_file("samples/brother/project263_part3.fcm");
}

#[test]
fn parses_brother_project264_part1() {
    test_file("samples/brother/project264_part1.fcm");
}

#[test]
fn parses_brother_project264_part2() {
    test_file("samples/brother/project264_part2.fcm");
}

#[test]
fn parses_brother_project264_part3() {
    test_file("samples/brother/project264_part3.fcm");
}

#[test]
fn parses_brother_project264_part4() {
    test_file("samples/brother/project264_part4.fcm");
}

#[test]
fn parses_brother_project264_part5() {
    test_file("samples/brother/project264_part5.fcm");
}

#[test]
fn parses_brother_project265_part1() {
    test_file("samples/brother/project265_part1.fcm");
}

#[test]
fn parses_brother_project265_part2() {
    test_file("samples/brother/project265_part2.fcm");
}

#[test]
fn parses_brother_project265_part3() {
    test_file("samples/brother/project265_part3.fcm");
}

#[test]
fn parses_brother_project266_part1() {
    test_file("samples/brother/project266_part1.fcm");
}

#[test]
fn parses_brother_project267_part1() {
    test_file("samples/brother/project267_part1.fcm");
}

#[test]
fn parses_brother_project267_part2() {
    test_file("samples/brother/project267_part2.fcm");
}

#[test]
fn parses_brother_project267_part3() {
    test_file("samples/brother/project267_part3.fcm");
}

#[test]
fn parses_brother_project268_part1() {
    test_file("samples/brother/project268_part1.fcm");
}

#[test]
fn parses_brother_project268_part2() {
    test_file("samples/brother/project268_part2.fcm");
}

#[test]
fn parses_brother_project269_part1() {
    test_file("samples/brother/project269_part1.fcm");
}

#[test]
fn parses_brother_project269_part2() {
    test_file("samples/brother/project269_part2.fcm");
}

#[test]
fn parses_brother_project269_part3() {
    test_file("samples/brother/project269_part3.fcm");
}

#[test]
fn parses_brother_project26_part1() {
    test_file("samples/brother/project26_part1.fcm");
}

#[test]
fn parses_brother_project26_part2() {
    test_file("samples/brother/project26_part2.fcm");
}

#[test]
fn parses_brother_project26_part3() {
    test_file("samples/brother/project26_part3.fcm");
}

#[test]
fn parses_brother_project26_part4() {
    test_file("samples/brother/project26_part4.fcm");
}

#[test]
fn parses_brother_project26_part5() {
    test_file("samples/brother/project26_part5.fcm");
}

#[test]
fn parses_brother_project270_part1() {
    test_file("samples/brother/project270_part1.fcm");
}

#[test]
fn parses_brother_project271_part1() {
    test_file("samples/brother/project271_part1.fcm");
}

#[test]
fn parses_brother_project271_part2() {
    test_file("samples/brother/project271_part2.fcm");
}

#[test]
fn parses_brother_project271_part3() {
    test_file("samples/brother/project271_part3.fcm");
}

#[test]
fn parses_brother_project272_part1() {
    test_file("samples/brother/project272_part1.fcm");
}

#[test]
fn parses_brother_project272_part2() {
    test_file("samples/brother/project272_part2.fcm");
}

#[test]
fn parses_brother_project272_part3() {
    test_file("samples/brother/project272_part3.fcm");
}

#[test]
fn parses_brother_project272_part4() {
    test_file("samples/brother/project272_part4.fcm");
}

#[test]
fn parses_brother_project273_part1() {
    test_file("samples/brother/project273_part1.fcm");
}

#[test]
fn parses_brother_project274_part1() {
    test_file("samples/brother/project274_part1.fcm");
}

#[test]
fn parses_brother_project275_part1() {
    test_file("samples/brother/project275_part1.fcm");
}

#[test]
fn parses_brother_project275_part2() {
    test_file("samples/brother/project275_part2.fcm");
}

#[test]
fn parses_brother_project275_part3() {
    test_file("samples/brother/project275_part3.fcm");
}

#[test]
fn parses_brother_project275_part4() {
    test_file("samples/brother/project275_part4.fcm");
}

#[test]
fn parses_brother_project275_part5() {
    test_file("samples/brother/project275_part5.fcm");
}

#[test]
fn parses_brother_project275_part6() {
    test_file("samples/brother/project275_part6.fcm");
}

#[test]
fn parses_brother_project276_part1() {
    test_file("samples/brother/project276_part1.fcm");
}

#[test]
fn parses_brother_project276_part2() {
    test_file("samples/brother/project276_part2.fcm");
}

#[test]
fn parses_brother_project276_part3() {
    test_file("samples/brother/project276_part3.fcm");
}

#[test]
fn parses_brother_project276_part4() {
    test_file("samples/brother/project276_part4.fcm");
}

#[test]
fn parses_brother_project276_part5() {
    test_file("samples/brother/project276_part5.fcm");
}

#[test]
fn parses_brother_project276_part6() {
    test_file("samples/brother/project276_part6.fcm");
}

#[test]
fn parses_brother_project276_part7() {
    test_file("samples/brother/project276_part7.fcm");
}

#[test]
fn parses_brother_project277_part1() {
    test_file("samples/brother/project277_part1.fcm");
}

#[test]
fn parses_brother_project278_part1() {
    test_file("samples/brother/project278_part1.fcm");
}

#[test]
fn parses_brother_project278_part2() {
    test_file("samples/brother/project278_part2.fcm");
}

#[test]
fn parses_brother_project278_part3() {
    test_file("samples/brother/project278_part3.fcm");
}

#[test]
fn parses_brother_project279_part1() {
    test_file("samples/brother/project279_part1.fcm");
}

#[test]
fn parses_brother_project27_part1() {
    test_file("samples/brother/project27_part1.fcm");
}

#[test]
fn parses_brother_project27_part2() {
    test_file("samples/brother/project27_part2.fcm");
}

#[test]
fn parses_brother_project27_part3() {
    test_file("samples/brother/project27_part3.fcm");
}

#[test]
fn parses_brother_project27_part4() {
    test_file("samples/brother/project27_part4.fcm");
}

#[test]
fn parses_brother_project27_part5() {
    test_file("samples/brother/project27_part5.fcm");
}

#[test]
fn parses_brother_project27_part6() {
    test_file("samples/brother/project27_part6.fcm");
}

#[test]
fn parses_brother_project27_part7() {
    test_file("samples/brother/project27_part7.fcm");
}

#[test]
fn parses_brother_project280_part1() {
    test_file("samples/brother/project280_part1.fcm");
}

#[test]
fn parses_brother_project280_part2() {
    test_file("samples/brother/project280_part2.fcm");
}

#[test]
fn parses_brother_project280_part3() {
    test_file("samples/brother/project280_part3.fcm");
}

#[test]
fn parses_brother_project280_part4() {
    test_file("samples/brother/project280_part4.fcm");
}

#[test]
fn parses_brother_project280_part5() {
    test_file("samples/brother/project280_part5.fcm");
}

#[test]
fn parses_brother_project280_part6() {
    test_file("samples/brother/project280_part6.fcm");
}

#[test]
fn parses_brother_project280_part7() {
    test_file("samples/brother/project280_part7.fcm");
}

#[test]
fn parses_brother_project281_part1() {
    test_file("samples/brother/project281_part1.fcm");
}

#[test]
fn parses_brother_project281_part2() {
    test_file("samples/brother/project281_part2.fcm");
}

#[test]
fn parses_brother_project282_part1() {
    test_file("samples/brother/project282_part1.fcm");
}

#[test]
fn parses_brother_project282_part2() {
    test_file("samples/brother/project282_part2.fcm");
}

#[test]
fn parses_brother_project282_part3() {
    test_file("samples/brother/project282_part3.fcm");
}

#[test]
fn parses_brother_project283_part1() {
    test_file("samples/brother/project283_part1.fcm");
}

#[test]
fn parses_brother_project284_part1() {
    test_file("samples/brother/project284_part1.fcm");
}

#[test]
fn parses_brother_project284_part2() {
    test_file("samples/brother/project284_part2.fcm");
}

#[test]
fn parses_brother_project284_part3() {
    test_file("samples/brother/project284_part3.fcm");
}

#[test]
fn parses_brother_project284_part4() {
    test_file("samples/brother/project284_part4.fcm");
}

#[test]
fn parses_brother_project285_part1() {
    test_file("samples/brother/project285_part1.fcm");
}

#[test]
fn parses_brother_project286_part1() {
    test_file("samples/brother/project286_part1.fcm");
}

#[test]
fn parses_brother_project286_part2() {
    test_file("samples/brother/project286_part2.fcm");
}

#[test]
fn parses_brother_project287_part1() {
    test_file("samples/brother/project287_part1.fcm");
}

#[test]
fn parses_brother_project287_part2() {
    test_file("samples/brother/project287_part2.fcm");
}

#[test]
fn parses_brother_project287_part3() {
    test_file("samples/brother/project287_part3.fcm");
}

#[test]
fn parses_brother_project287_part4() {
    test_file("samples/brother/project287_part4.fcm");
}

#[test]
fn parses_brother_project288_part1() {
    test_file("samples/brother/project288_part1.fcm");
}

#[test]
fn parses_brother_project288_part2() {
    test_file("samples/brother/project288_part2.fcm");
}

#[test]
fn parses_brother_project288_part3() {
    test_file("samples/brother/project288_part3.fcm");
}

#[test]
fn parses_brother_project288_part4() {
    test_file("samples/brother/project288_part4.fcm");
}

#[test]
fn parses_brother_project289_part1() {
    test_file("samples/brother/project289_part1.fcm");
}

#[test]
fn parses_brother_project289_part2() {
    test_file("samples/brother/project289_part2.fcm");
}

#[test]
fn parses_brother_project28_part1() {
    test_file("samples/brother/project28_part1.fcm");
}

#[test]
fn parses_brother_project290_part1() {
    test_file("samples/brother/project290_part1.fcm");
}

#[test]
fn parses_brother_project290_part2() {
    test_file("samples/brother/project290_part2.fcm");
}

#[test]
fn parses_brother_project290_part3() {
    test_file("samples/brother/project290_part3.fcm");
}

#[test]
fn parses_brother_project291_part1() {
    test_file("samples/brother/project291_part1.fcm");
}

#[test]
fn parses_brother_project291_part2() {
    test_file("samples/brother/project291_part2.fcm");
}

#[test]
fn parses_brother_project291_part3() {
    test_file("samples/brother/project291_part3.fcm");
}

#[test]
fn parses_brother_project291_part4() {
    test_file("samples/brother/project291_part4.fcm");
}

#[test]
fn parses_brother_project292_part10() {
    test_file("samples/brother/project292_part10.fcm");
}

#[test]
fn parses_brother_project292_part1() {
    test_file("samples/brother/project292_part1.fcm");
}

#[test]
fn parses_brother_project292_part2() {
    test_file("samples/brother/project292_part2.fcm");
}

#[test]
fn parses_brother_project292_part3() {
    test_file("samples/brother/project292_part3.fcm");
}

#[test]
fn parses_brother_project292_part4() {
    test_file("samples/brother/project292_part4.fcm");
}

#[test]
fn parses_brother_project292_part5() {
    test_file("samples/brother/project292_part5.fcm");
}

#[test]
fn parses_brother_project292_part6() {
    test_file("samples/brother/project292_part6.fcm");
}

#[test]
fn parses_brother_project292_part7() {
    test_file("samples/brother/project292_part7.fcm");
}

#[test]
fn parses_brother_project292_part8() {
    test_file("samples/brother/project292_part8.fcm");
}

#[test]
fn parses_brother_project292_part9() {
    test_file("samples/brother/project292_part9.fcm");
}

#[test]
fn parses_brother_project293_part1() {
    test_file("samples/brother/project293_part1.fcm");
}

#[test]
fn parses_brother_project293_part2() {
    test_file("samples/brother/project293_part2.fcm");
}

#[test]
fn parses_brother_project293_part3() {
    test_file("samples/brother/project293_part3.fcm");
}

#[test]
fn parses_brother_project293_part4() {
    test_file("samples/brother/project293_part4.fcm");
}

#[test]
fn parses_brother_project293_part5() {
    test_file("samples/brother/project293_part5.fcm");
}

#[test]
fn parses_brother_project295_part1() {
    test_file("samples/brother/project295_part1.fcm");
}

#[test]
fn parses_brother_project295_part2() {
    test_file("samples/brother/project295_part2.fcm");
}

#[test]
fn parses_brother_project295_part3() {
    test_file("samples/brother/project295_part3.fcm");
}

#[test]
fn parses_brother_project295_part4() {
    test_file("samples/brother/project295_part4.fcm");
}

#[test]
fn parses_brother_project296_part1() {
    test_file("samples/brother/project296_part1.fcm");
}

#[test]
fn parses_brother_project297_part1() {
    test_file("samples/brother/project297_part1.fcm");
}

#[test]
fn parses_brother_project298_part1() {
    test_file("samples/brother/project298_part1.fcm");
}

#[test]
fn parses_brother_project298_part2() {
    test_file("samples/brother/project298_part2.fcm");
}

#[test]
fn parses_brother_project299_part1() {
    test_file("samples/brother/project299_part1.fcm");
}

#[test]
fn parses_brother_project299_part2() {
    test_file("samples/brother/project299_part2.fcm");
}

#[test]
fn parses_brother_project299_part3() {
    test_file("samples/brother/project299_part3.fcm");
}

#[test]
fn parses_brother_project29_part1() {
    test_file("samples/brother/project29_part1.fcm");
}

#[test]
fn parses_brother_project29_part2() {
    test_file("samples/brother/project29_part2.fcm");
}

#[test]
fn parses_brother_project29_part3() {
    test_file("samples/brother/project29_part3.fcm");
}

#[test]
fn parses_brother_project2_part1() {
    test_file("samples/brother/project2_part1.fcm");
}

#[test]
fn parses_brother_project2_part2() {
    test_file("samples/brother/project2_part2.fcm");
}

#[test]
fn parses_brother_project2_part3() {
    test_file("samples/brother/project2_part3.fcm");
}

#[test]
fn parses_brother_project300_part1() {
    test_file("samples/brother/project300_part1.fcm");
}

#[test]
fn parses_brother_project301_part1() {
    test_file("samples/brother/project301_part1.fcm");
}

#[test]
fn parses_brother_project302_part1() {
    test_file("samples/brother/project302_part1.fcm");
}

#[test]
fn parses_brother_project303_part1() {
    test_file("samples/brother/project303_part1.fcm");
}

#[test]
fn parses_brother_project304_part1() {
    test_file("samples/brother/project304_part1.fcm");
}

#[test]
fn parses_brother_project304_part2() {
    test_file("samples/brother/project304_part2.fcm");
}

#[test]
fn parses_brother_project304_part3() {
    test_file("samples/brother/project304_part3.fcm");
}

#[test]
fn parses_brother_project304_part4() {
    test_file("samples/brother/project304_part4.fcm");
}

#[test]
fn parses_brother_project305_part1() {
    test_file("samples/brother/project305_part1.fcm");
}

#[test]
fn parses_brother_project305_part2() {
    test_file("samples/brother/project305_part2.fcm");
}

#[test]
fn parses_brother_project306_part1() {
    test_file("samples/brother/project306_part1.fcm");
}

#[test]
fn parses_brother_project307_part1() {
    test_file("samples/brother/project307_part1.fcm");
}

#[test]
fn parses_brother_project308_part1() {
    test_file("samples/brother/project308_part1.fcm");
}

#[test]
fn parses_brother_project308_part2() {
    test_file("samples/brother/project308_part2.fcm");
}

#[test]
fn parses_brother_project309_part1() {
    test_file("samples/brother/project309_part1.fcm");
}

#[test]
fn parses_brother_project309_part2() {
    test_file("samples/brother/project309_part2.fcm");
}

#[test]
fn parses_brother_project309_part3() {
    test_file("samples/brother/project309_part3.fcm");
}

#[test]
fn parses_brother_project30_part1() {
    test_file("samples/brother/project30_part1.fcm");
}

#[test]
fn parses_brother_project310_part1() {
    test_file("samples/brother/project310_part1.fcm");
}

#[test]
fn parses_brother_project311_part1() {
    test_file("samples/brother/project311_part1.fcm");
}

#[test]
fn parses_brother_project311_part2() {
    test_file("samples/brother/project311_part2.fcm");
}

#[test]
fn parses_brother_project312_part1() {
    test_file("samples/brother/project312_part1.fcm");
}

#[test]
fn parses_brother_project312_part2() {
    test_file("samples/brother/project312_part2.fcm");
}

#[test]
fn parses_brother_project312_part3() {
    test_file("samples/brother/project312_part3.fcm");
}

#[test]
fn parses_brother_project312_part4() {
    test_file("samples/brother/project312_part4.fcm");
}

#[test]
fn parses_brother_project313_part1() {
    test_file("samples/brother/project313_part1.fcm");
}

#[test]
fn parses_brother_project313_part2() {
    test_file("samples/brother/project313_part2.fcm");
}

#[test]
fn parses_brother_project313_part3() {
    test_file("samples/brother/project313_part3.fcm");
}

#[test]
fn parses_brother_project314_part1() {
    test_file("samples/brother/project314_part1.fcm");
}

#[test]
fn parses_brother_project315_part1() {
    test_file("samples/brother/project315_part1.fcm");
}

#[test]
fn parses_brother_project316_part1() {
    test_file("samples/brother/project316_part1.fcm");
}

#[test]
fn parses_brother_project316_part2() {
    test_file("samples/brother/project316_part2.fcm");
}

#[test]
fn parses_brother_project316_part3() {
    test_file("samples/brother/project316_part3.fcm");
}

#[test]
fn parses_brother_project317_part1() {
    test_file("samples/brother/project317_part1.fcm");
}

#[test]
fn parses_brother_project317_part2() {
    test_file("samples/brother/project317_part2.fcm");
}

#[test]
fn parses_brother_project318_part1() {
    test_file("samples/brother/project318_part1.fcm");
}

#[test]
fn parses_brother_project319_part1() {
    test_file("samples/brother/project319_part1.fcm");
}

#[test]
fn parses_brother_project319_part2() {
    test_file("samples/brother/project319_part2.fcm");
}

#[test]
fn parses_brother_project31_part1() {
    test_file("samples/brother/project31_part1.fcm");
}

#[test]
fn parses_brother_project320_part1() {
    test_file("samples/brother/project320_part1.fcm");
}

#[test]
fn parses_brother_project320_part2() {
    test_file("samples/brother/project320_part2.fcm");
}

#[test]
fn parses_brother_project321_part1() {
    test_file("samples/brother/project321_part1.fcm");
}

#[test]
fn parses_brother_project321_part2() {
    test_file("samples/brother/project321_part2.fcm");
}

#[test]
fn parses_brother_project322_part1() {
    test_file("samples/brother/project322_part1.fcm");
}

#[test]
fn parses_brother_project322_part2() {
    test_file("samples/brother/project322_part2.fcm");
}

#[test]
fn parses_brother_project322_part3() {
    test_file("samples/brother/project322_part3.fcm");
}

#[test]
fn parses_brother_project323_part1() {
    test_file("samples/brother/project323_part1.fcm");
}

#[test]
fn parses_brother_project323_part2() {
    test_file("samples/brother/project323_part2.fcm");
}

#[test]
fn parses_brother_project323_part3() {
    test_file("samples/brother/project323_part3.fcm");
}

#[test]
fn parses_brother_project324_part1() {
    test_file("samples/brother/project324_part1.fcm");
}

#[test]
fn parses_brother_project324_part2() {
    test_file("samples/brother/project324_part2.fcm");
}

#[test]
fn parses_brother_project324_part3() {
    test_file("samples/brother/project324_part3.fcm");
}

#[test]
fn parses_brother_project325_part1() {
    test_file("samples/brother/project325_part1.fcm");
}

#[test]
fn parses_brother_project326_part1() {
    test_file("samples/brother/project326_part1.fcm");
}

#[test]
fn parses_brother_project326_part2() {
    test_file("samples/brother/project326_part2.fcm");
}

#[test]
fn parses_brother_project326_part3() {
    test_file("samples/brother/project326_part3.fcm");
}

#[test]
fn parses_brother_project326_part4() {
    test_file("samples/brother/project326_part4.fcm");
}

#[test]
fn parses_brother_project327_part1() {
    test_file("samples/brother/project327_part1.fcm");
}

#[test]
fn parses_brother_project327_part2() {
    test_file("samples/brother/project327_part2.fcm");
}

#[test]
fn parses_brother_project328_part1() {
    test_file("samples/brother/project328_part1.fcm");
}

#[test]
fn parses_brother_project328_part2() {
    test_file("samples/brother/project328_part2.fcm");
}

#[test]
fn parses_brother_project328_part3() {
    test_file("samples/brother/project328_part3.fcm");
}

#[test]
fn parses_brother_project329_part1() {
    test_file("samples/brother/project329_part1.fcm");
}

#[test]
fn parses_brother_project329_part2() {
    test_file("samples/brother/project329_part2.fcm");
}

#[test]
fn parses_brother_project32_part1() {
    test_file("samples/brother/project32_part1.fcm");
}

#[test]
fn parses_brother_project32_part2() {
    test_file("samples/brother/project32_part2.fcm");
}

#[test]
fn parses_brother_project32_part3() {
    test_file("samples/brother/project32_part3.fcm");
}

#[test]
fn parses_brother_project330_part1() {
    test_file("samples/brother/project330_part1.fcm");
}

#[test]
fn parses_brother_project330_part2() {
    test_file("samples/brother/project330_part2.fcm");
}

#[test]
fn parses_brother_project330_part3() {
    test_file("samples/brother/project330_part3.fcm");
}

#[test]
fn parses_brother_project330_part4() {
    test_file("samples/brother/project330_part4.fcm");
}

#[test]
fn parses_brother_project330_part5() {
    test_file("samples/brother/project330_part5.fcm");
}

#[test]
fn parses_brother_project331_part1() {
    test_file("samples/brother/project331_part1.fcm");
}

#[test]
fn parses_brother_project332_part1() {
    test_file("samples/brother/project332_part1.fcm");
}

#[test]
fn parses_brother_project333_part1() {
    test_file("samples/brother/project333_part1.fcm");
}

#[test]
fn parses_brother_project333_part2() {
    test_file("samples/brother/project333_part2.fcm");
}

#[test]
fn parses_brother_project334_part1() {
    test_file("samples/brother/project334_part1.fcm");
}

#[test]
fn parses_brother_project334_part2() {
    test_file("samples/brother/project334_part2.fcm");
}

#[test]
fn parses_brother_project335_part1() {
    test_file("samples/brother/project335_part1.fcm");
}

#[test]
fn parses_brother_project335_part2() {
    test_file("samples/brother/project335_part2.fcm");
}

#[test]
fn parses_brother_project335_part3() {
    test_file("samples/brother/project335_part3.fcm");
}

#[test]
fn parses_brother_project335_part4() {
    test_file("samples/brother/project335_part4.fcm");
}

#[test]
fn parses_brother_project336_part1() {
    test_file("samples/brother/project336_part1.fcm");
}

#[test]
fn parses_brother_project336_part2() {
    test_file("samples/brother/project336_part2.fcm");
}

#[test]
fn parses_brother_project336_part3() {
    test_file("samples/brother/project336_part3.fcm");
}

#[test]
fn parses_brother_project336_part4() {
    test_file("samples/brother/project336_part4.fcm");
}

#[test]
fn parses_brother_project337_part1() {
    test_file("samples/brother/project337_part1.fcm");
}

#[test]
fn parses_brother_project337_part2() {
    test_file("samples/brother/project337_part2.fcm");
}

#[test]
fn parses_brother_project337_part3() {
    test_file("samples/brother/project337_part3.fcm");
}

#[test]
fn parses_brother_project337_part4() {
    test_file("samples/brother/project337_part4.fcm");
}

#[test]
fn parses_brother_project338_part1() {
    test_file("samples/brother/project338_part1.fcm");
}

#[test]
fn parses_brother_project338_part2() {
    test_file("samples/brother/project338_part2.fcm");
}

#[test]
fn parses_brother_project338_part3() {
    test_file("samples/brother/project338_part3.fcm");
}

#[test]
fn parses_brother_project338_part4() {
    test_file("samples/brother/project338_part4.fcm");
}

#[test]
fn parses_brother_project338_part5() {
    test_file("samples/brother/project338_part5.fcm");
}

#[test]
fn parses_brother_project339_part1() {
    test_file("samples/brother/project339_part1.fcm");
}

#[test]
fn parses_brother_project33_part1() {
    test_file("samples/brother/project33_part1.fcm");
}

#[test]
fn parses_brother_project33_part2() {
    test_file("samples/brother/project33_part2.fcm");
}

#[test]
fn parses_brother_project33_part3() {
    test_file("samples/brother/project33_part3.fcm");
}

#[test]
fn parses_brother_project33_part4() {
    test_file("samples/brother/project33_part4.fcm");
}

#[test]
fn parses_brother_project340_part1() {
    test_file("samples/brother/project340_part1.fcm");
}

#[test]
fn parses_brother_project340_part2() {
    test_file("samples/brother/project340_part2.fcm");
}

#[test]
fn parses_brother_project341_part1() {
    test_file("samples/brother/project341_part1.fcm");
}

#[test]
fn parses_brother_project341_part2() {
    test_file("samples/brother/project341_part2.fcm");
}

#[test]
fn parses_brother_project342_part1() {
    test_file("samples/brother/project342_part1.fcm");
}

#[test]
fn parses_brother_project342_part2() {
    test_file("samples/brother/project342_part2.fcm");
}

#[test]
fn parses_brother_project343_part10() {
    test_file("samples/brother/project343_part10.fcm");
}

#[test]
fn parses_brother_project343_part11() {
    test_file("samples/brother/project343_part11.fcm");
}

#[test]
fn parses_brother_project343_part1() {
    test_file("samples/brother/project343_part1.fcm");
}

#[test]
fn parses_brother_project343_part2() {
    test_file("samples/brother/project343_part2.fcm");
}

#[test]
fn parses_brother_project343_part3() {
    test_file("samples/brother/project343_part3.fcm");
}

#[test]
fn parses_brother_project343_part4() {
    test_file("samples/brother/project343_part4.fcm");
}

#[test]
fn parses_brother_project343_part5() {
    test_file("samples/brother/project343_part5.fcm");
}

#[test]
fn parses_brother_project343_part6() {
    test_file("samples/brother/project343_part6.fcm");
}

#[test]
fn parses_brother_project343_part7() {
    test_file("samples/brother/project343_part7.fcm");
}

#[test]
fn parses_brother_project343_part8() {
    test_file("samples/brother/project343_part8.fcm");
}

#[test]
fn parses_brother_project343_part9() {
    test_file("samples/brother/project343_part9.fcm");
}

#[test]
fn parses_brother_project344_part1() {
    test_file("samples/brother/project344_part1.fcm");
}

#[test]
fn parses_brother_project345_part1() {
    test_file("samples/brother/project345_part1.fcm");
}

#[test]
fn parses_brother_project345_part2() {
    test_file("samples/brother/project345_part2.fcm");
}

#[test]
fn parses_brother_project346_part1() {
    test_file("samples/brother/project346_part1.fcm");
}

#[test]
fn parses_brother_project346_part2() {
    test_file("samples/brother/project346_part2.fcm");
}

#[test]
fn parses_brother_project346_part3() {
    test_file("samples/brother/project346_part3.fcm");
}

#[test]
fn parses_brother_project347_part1() {
    test_file("samples/brother/project347_part1.fcm");
}

#[test]
fn parses_brother_project348_part1() {
    test_file("samples/brother/project348_part1.fcm");
}

#[test]
fn parses_brother_project348_part2() {
    test_file("samples/brother/project348_part2.fcm");
}

#[test]
fn parses_brother_project349_part1() {
    test_file("samples/brother/project349_part1.fcm");
}

#[test]
fn parses_brother_project34_part1() {
    test_file("samples/brother/project34_part1.fcm");
}

#[test]
fn parses_brother_project34_part2() {
    test_file("samples/brother/project34_part2.fcm");
}

#[test]
fn parses_brother_project34_part3() {
    test_file("samples/brother/project34_part3.fcm");
}

#[test]
fn parses_brother_project350_part1() {
    test_file("samples/brother/project350_part1.fcm");
}

#[test]
fn parses_brother_project350_part2() {
    test_file("samples/brother/project350_part2.fcm");
}

#[test]
fn parses_brother_project351_part1() {
    test_file("samples/brother/project351_part1.fcm");
}

#[test]
fn parses_brother_project351_part2() {
    test_file("samples/brother/project351_part2.fcm");
}

#[test]
fn parses_brother_project351_part3() {
    test_file("samples/brother/project351_part3.fcm");
}

#[test]
fn parses_brother_project351_part4() {
    test_file("samples/brother/project351_part4.fcm");
}

#[test]
fn parses_brother_project351_part5() {
    test_file("samples/brother/project351_part5.fcm");
}

#[test]
fn parses_brother_project352_part1() {
    test_file("samples/brother/project352_part1.fcm");
}

#[test]
fn parses_brother_project352_part2() {
    test_file("samples/brother/project352_part2.fcm");
}

#[test]
fn parses_brother_project353_part1() {
    test_file("samples/brother/project353_part1.fcm");
}

#[test]
fn parses_brother_project353_part2() {
    test_file("samples/brother/project353_part2.fcm");
}

#[test]
fn parses_brother_project353_part3() {
    test_file("samples/brother/project353_part3.fcm");
}

#[test]
fn parses_brother_project354_part1() {
    test_file("samples/brother/project354_part1.fcm");
}

#[test]
fn parses_brother_project354_part2() {
    test_file("samples/brother/project354_part2.fcm");
}

#[test]
fn parses_brother_project354_part3() {
    test_file("samples/brother/project354_part3.fcm");
}

#[test]
fn parses_brother_project354_part4() {
    test_file("samples/brother/project354_part4.fcm");
}

#[test]
fn parses_brother_project354_part5() {
    test_file("samples/brother/project354_part5.fcm");
}

#[test]
fn parses_brother_project355_part1() {
    test_file("samples/brother/project355_part1.fcm");
}

#[test]
fn parses_brother_project355_part2() {
    test_file("samples/brother/project355_part2.fcm");
}

#[test]
fn parses_brother_project355_part3() {
    test_file("samples/brother/project355_part3.fcm");
}

#[test]
fn parses_brother_project356_part1() {
    test_file("samples/brother/project356_part1.fcm");
}

#[test]
fn parses_brother_project356_part2() {
    test_file("samples/brother/project356_part2.fcm");
}

#[test]
fn parses_brother_project357_part1() {
    test_file("samples/brother/project357_part1.fcm");
}

#[test]
fn parses_brother_project358_part1() {
    test_file("samples/brother/project358_part1.fcm");
}

#[test]
fn parses_brother_project358_part2() {
    test_file("samples/brother/project358_part2.fcm");
}

#[test]
fn parses_brother_project358_part3() {
    test_file("samples/brother/project358_part3.fcm");
}

#[test]
fn parses_brother_project358_part4() {
    test_file("samples/brother/project358_part4.fcm");
}

#[test]
fn parses_brother_project359_part1() {
    test_file("samples/brother/project359_part1.fcm");
}

#[test]
fn parses_brother_project359_part2() {
    test_file("samples/brother/project359_part2.fcm");
}

#[test]
fn parses_brother_project359_part3() {
    test_file("samples/brother/project359_part3.fcm");
}

#[test]
fn parses_brother_project359_part4() {
    test_file("samples/brother/project359_part4.fcm");
}

#[test]
fn parses_brother_project35_part1() {
    test_file("samples/brother/project35_part1.fcm");
}

#[test]
fn parses_brother_project35_part2() {
    test_file("samples/brother/project35_part2.fcm");
}

#[test]
fn parses_brother_project35_part3() {
    test_file("samples/brother/project35_part3.fcm");
}

#[test]
fn parses_brother_project360_part1() {
    test_file("samples/brother/project360_part1.fcm");
}

#[test]
fn parses_brother_project360_part2() {
    test_file("samples/brother/project360_part2.fcm");
}

#[test]
fn parses_brother_project360_part3() {
    test_file("samples/brother/project360_part3.fcm");
}

#[test]
fn parses_brother_project360_part4() {
    test_file("samples/brother/project360_part4.fcm");
}

#[test]
fn parses_brother_project361_part1() {
    test_file("samples/brother/project361_part1.fcm");
}

#[test]
fn parses_brother_project362_part1() {
    test_file("samples/brother/project362_part1.fcm");
}

#[test]
fn parses_brother_project362_part2() {
    test_file("samples/brother/project362_part2.fcm");
}

#[test]
fn parses_brother_project362_part3() {
    test_file("samples/brother/project362_part3.fcm");
}

#[test]
fn parses_brother_project362_part4() {
    test_file("samples/brother/project362_part4.fcm");
}

#[test]
fn parses_brother_project363_part1() {
    test_file("samples/brother/project363_part1.fcm");
}

#[test]
fn parses_brother_project363_part2() {
    test_file("samples/brother/project363_part2.fcm");
}

#[test]
fn parses_brother_project363_part3() {
    test_file("samples/brother/project363_part3.fcm");
}

#[test]
fn parses_brother_project36_part1() {
    test_file("samples/brother/project36_part1.fcm");
}

#[test]
fn parses_brother_project36_part2() {
    test_file("samples/brother/project36_part2.fcm");
}

#[test]
fn parses_brother_project37_part1() {
    test_file("samples/brother/project37_part1.fcm");
}

#[test]
fn parses_brother_project37_part2() {
    test_file("samples/brother/project37_part2.fcm");
}

#[test]
fn parses_brother_project37_part3() {
    test_file("samples/brother/project37_part3.fcm");
}

#[test]
fn parses_brother_project37_part4() {
    test_file("samples/brother/project37_part4.fcm");
}

#[test]
fn parses_brother_project37_part5() {
    test_file("samples/brother/project37_part5.fcm");
}

#[test]
fn parses_brother_project37_part6() {
    test_file("samples/brother/project37_part6.fcm");
}

#[test]
fn parses_brother_project37_part7() {
    test_file("samples/brother/project37_part7.fcm");
}

#[test]
fn parses_brother_project38_part1() {
    test_file("samples/brother/project38_part1.fcm");
}

#[test]
fn parses_brother_project39_part1() {
    test_file("samples/brother/project39_part1.fcm");
}

#[test]
fn parses_brother_project3_part1() {
    test_file("samples/brother/project3_part1.fcm");
}

#[test]
fn parses_brother_project3_part2() {
    test_file("samples/brother/project3_part2.fcm");
}

#[test]
fn parses_brother_project3_part3() {
    test_file("samples/brother/project3_part3.fcm");
}

#[test]
fn parses_brother_project3_part4() {
    test_file("samples/brother/project3_part4.fcm");
}

#[test]
fn parses_brother_project3_part5() {
    test_file("samples/brother/project3_part5.fcm");
}

#[test]
fn parses_brother_project3_part6() {
    test_file("samples/brother/project3_part6.fcm");
}

#[test]
fn parses_brother_project3_part7() {
    test_file("samples/brother/project3_part7.fcm");
}

#[test]
fn parses_brother_project40_part1() {
    test_file("samples/brother/project40_part1.fcm");
}

#[test]
fn parses_brother_project40_part2() {
    test_file("samples/brother/project40_part2.fcm");
}

#[test]
fn parses_brother_project41_part1() {
    test_file("samples/brother/project41_part1.fcm");
}

#[test]
fn parses_brother_project41_part2() {
    test_file("samples/brother/project41_part2.fcm");
}

#[test]
fn parses_brother_project41_part3() {
    test_file("samples/brother/project41_part3.fcm");
}

#[test]
fn parses_brother_project42_part1() {
    test_file("samples/brother/project42_part1.fcm");
}

#[test]
fn parses_brother_project42_part2() {
    test_file("samples/brother/project42_part2.fcm");
}

#[test]
fn parses_brother_project42_part3() {
    test_file("samples/brother/project42_part3.fcm");
}

#[test]
fn parses_brother_project42_part4() {
    test_file("samples/brother/project42_part4.fcm");
}

#[test]
fn parses_brother_project43_part1() {
    test_file("samples/brother/project43_part1.fcm");
}

#[test]
fn parses_brother_project44_part1() {
    test_file("samples/brother/project44_part1.fcm");
}

#[test]
fn parses_brother_project45_part1() {
    test_file("samples/brother/project45_part1.fcm");
}

#[test]
fn parses_brother_project46_part1() {
    test_file("samples/brother/project46_part1.fcm");
}

#[test]
fn parses_brother_project46_part2() {
    test_file("samples/brother/project46_part2.fcm");
}

#[test]
fn parses_brother_project46_part3() {
    test_file("samples/brother/project46_part3.fcm");
}

#[test]
fn parses_brother_project46_part4() {
    test_file("samples/brother/project46_part4.fcm");
}

#[test]
fn parses_brother_project47_part1() {
    test_file("samples/brother/project47_part1.fcm");
}

#[test]
fn parses_brother_project47_part2() {
    test_file("samples/brother/project47_part2.fcm");
}

#[test]
fn parses_brother_project48_part1() {
    test_file("samples/brother/project48_part1.fcm");
}

#[test]
fn parses_brother_project49_part1() {
    test_file("samples/brother/project49_part1.fcm");
}

#[test]
fn parses_brother_project49_part2() {
    test_file("samples/brother/project49_part2.fcm");
}

#[test]
fn parses_brother_project4_part1() {
    test_file("samples/brother/project4_part1.fcm");
}

#[test]
fn parses_brother_project50_part1() {
    test_file("samples/brother/project50_part1.fcm");
}

#[test]
fn parses_brother_project51_part1() {
    test_file("samples/brother/project51_part1.fcm");
}

#[test]
fn parses_brother_project51_part2() {
    test_file("samples/brother/project51_part2.fcm");
}

#[test]
fn parses_brother_project51_part3() {
    test_file("samples/brother/project51_part3.fcm");
}

#[test]
fn parses_brother_project52_part1() {
    test_file("samples/brother/project52_part1.fcm");
}

#[test]
fn parses_brother_project52_part2() {
    test_file("samples/brother/project52_part2.fcm");
}

#[test]
fn parses_brother_project52_part3() {
    test_file("samples/brother/project52_part3.fcm");
}

#[test]
fn parses_brother_project52_part4() {
    test_file("samples/brother/project52_part4.fcm");
}

#[test]
fn parses_brother_project53_part1() {
    test_file("samples/brother/project53_part1.fcm");
}

#[test]
fn parses_brother_project53_part2() {
    test_file("samples/brother/project53_part2.fcm");
}

#[test]
fn parses_brother_project53_part3() {
    test_file("samples/brother/project53_part3.fcm");
}

#[test]
fn parses_brother_project54_part1() {
    test_file("samples/brother/project54_part1.fcm");
}

#[test]
fn parses_brother_project55_part1() {
    test_file("samples/brother/project55_part1.fcm");
}

#[test]
fn parses_brother_project55_part2() {
    test_file("samples/brother/project55_part2.fcm");
}

#[test]
fn parses_brother_project55_part3() {
    test_file("samples/brother/project55_part3.fcm");
}

#[test]
fn parses_brother_project56_part1() {
    test_file("samples/brother/project56_part1.fcm");
}

#[test]
fn parses_brother_project56_part2() {
    test_file("samples/brother/project56_part2.fcm");
}

#[test]
fn parses_brother_project56_part3() {
    test_file("samples/brother/project56_part3.fcm");
}

#[test]
fn parses_brother_project56_part4() {
    test_file("samples/brother/project56_part4.fcm");
}

#[test]
fn parses_brother_project57_part1() {
    test_file("samples/brother/project57_part1.fcm");
}

#[test]
fn parses_brother_project58_part1() {
    test_file("samples/brother/project58_part1.fcm");
}

#[test]
fn parses_brother_project59_part1() {
    test_file("samples/brother/project59_part1.fcm");
}

#[test]
fn parses_brother_project5_part10() {
    test_file("samples/brother/project5_part10.fcm");
}

#[test]
fn parses_brother_project5_part11() {
    test_file("samples/brother/project5_part11.fcm");
}

#[test]
fn parses_brother_project5_part12() {
    test_file("samples/brother/project5_part12.fcm");
}

#[test]
fn parses_brother_project5_part13() {
    test_file("samples/brother/project5_part13.fcm");
}

#[test]
fn parses_brother_project5_part14() {
    test_file("samples/brother/project5_part14.fcm");
}

#[test]
fn parses_brother_project5_part15() {
    test_file("samples/brother/project5_part15.fcm");
}

#[test]
fn parses_brother_project5_part16() {
    test_file("samples/brother/project5_part16.fcm");
}

#[test]
fn parses_brother_project5_part1() {
    test_file("samples/brother/project5_part1.fcm");
}

#[test]
fn parses_brother_project5_part2() {
    test_file("samples/brother/project5_part2.fcm");
}

#[test]
fn parses_brother_project5_part3() {
    test_file("samples/brother/project5_part3.fcm");
}

#[test]
fn parses_brother_project5_part4() {
    test_file("samples/brother/project5_part4.fcm");
}

#[test]
fn parses_brother_project5_part5() {
    test_file("samples/brother/project5_part5.fcm");
}

#[test]
fn parses_brother_project5_part6() {
    test_file("samples/brother/project5_part6.fcm");
}

#[test]
fn parses_brother_project5_part7() {
    test_file("samples/brother/project5_part7.fcm");
}

#[test]
fn parses_brother_project5_part8() {
    test_file("samples/brother/project5_part8.fcm");
}

#[test]
fn parses_brother_project5_part9() {
    test_file("samples/brother/project5_part9.fcm");
}

#[test]
fn parses_brother_project60_part1() {
    test_file("samples/brother/project60_part1.fcm");
}

#[test]
fn parses_brother_project60_part2() {
    test_file("samples/brother/project60_part2.fcm");
}

#[test]
fn parses_brother_project61_part1() {
    test_file("samples/brother/project61_part1.fcm");
}

#[test]
fn parses_brother_project61_part2() {
    test_file("samples/brother/project61_part2.fcm");
}

#[test]
fn parses_brother_project61_part3() {
    test_file("samples/brother/project61_part3.fcm");
}

#[test]
fn parses_brother_project61_part4() {
    test_file("samples/brother/project61_part4.fcm");
}

#[test]
fn parses_brother_project62_part1() {
    test_file("samples/brother/project62_part1.fcm");
}

#[test]
fn parses_brother_project62_part2() {
    test_file("samples/brother/project62_part2.fcm");
}

#[test]
fn parses_brother_project62_part3() {
    test_file("samples/brother/project62_part3.fcm");
}

#[test]
fn parses_brother_project63_part1() {
    test_file("samples/brother/project63_part1.fcm");
}

#[test]
fn parses_brother_project64_part1() {
    test_file("samples/brother/project64_part1.fcm");
}

#[test]
fn parses_brother_project64_part2() {
    test_file("samples/brother/project64_part2.fcm");
}

#[test]
fn parses_brother_project64_part3() {
    test_file("samples/brother/project64_part3.fcm");
}

#[test]
fn parses_brother_project65_part1() {
    test_file("samples/brother/project65_part1.fcm");
}

#[test]
fn parses_brother_project65_part2() {
    test_file("samples/brother/project65_part2.fcm");
}

#[test]
fn parses_brother_project66_part1() {
    test_file("samples/brother/project66_part1.fcm");
}

#[test]
fn parses_brother_project66_part2() {
    test_file("samples/brother/project66_part2.fcm");
}

#[test]
fn parses_brother_project66_part3() {
    test_file("samples/brother/project66_part3.fcm");
}

#[test]
fn parses_brother_project67_part1() {
    test_file("samples/brother/project67_part1.fcm");
}

#[test]
fn parses_brother_project67_part2() {
    test_file("samples/brother/project67_part2.fcm");
}

#[test]
fn parses_brother_project68_part1() {
    test_file("samples/brother/project68_part1.fcm");
}

#[test]
fn parses_brother_project68_part2() {
    test_file("samples/brother/project68_part2.fcm");
}

#[test]
fn parses_brother_project68_part3() {
    test_file("samples/brother/project68_part3.fcm");
}

#[test]
fn parses_brother_project69_part1() {
    test_file("samples/brother/project69_part1.fcm");
}

#[test]
fn parses_brother_project69_part3() {
    test_file("samples/brother/project69_part3.fcm");
}

#[test]
fn parses_brother_project69_part4() {
    test_file("samples/brother/project69_part4.fcm");
}

#[test]
fn parses_brother_project6_part1() {
    test_file("samples/brother/project6_part1.fcm");
}

#[test]
fn parses_brother_project6_part2() {
    test_file("samples/brother/project6_part2.fcm");
}

#[test]
fn parses_brother_project6_part3() {
    test_file("samples/brother/project6_part3.fcm");
}

#[test]
fn parses_brother_project6_part4() {
    test_file("samples/brother/project6_part4.fcm");
}

#[test]
fn parses_brother_project70_part1() {
    test_file("samples/brother/project70_part1.fcm");
}

#[test]
fn parses_brother_project70_part2() {
    test_file("samples/brother/project70_part2.fcm");
}

#[test]
fn parses_brother_project70_part3() {
    test_file("samples/brother/project70_part3.fcm");
}

#[test]
fn parses_brother_project71_part1() {
    test_file("samples/brother/project71_part1.fcm");
}

#[test]
fn parses_brother_project72_part1() {
    test_file("samples/brother/project72_part1.fcm");
}

#[test]
fn parses_brother_project73_part1() {
    test_file("samples/brother/project73_part1.fcm");
}

#[test]
fn parses_brother_project73_part2() {
    test_file("samples/brother/project73_part2.fcm");
}

#[test]
fn parses_brother_project73_part3() {
    test_file("samples/brother/project73_part3.fcm");
}

#[test]
fn parses_brother_project73_part4() {
    test_file("samples/brother/project73_part4.fcm");
}

#[test]
fn parses_brother_project74_part1() {
    test_file("samples/brother/project74_part1.fcm");
}

#[test]
fn parses_brother_project74_part2() {
    test_file("samples/brother/project74_part2.fcm");
}

#[test]
fn parses_brother_project74_part3() {
    test_file("samples/brother/project74_part3.fcm");
}

#[test]
fn parses_brother_project74_part4() {
    test_file("samples/brother/project74_part4.fcm");
}

#[test]
fn parses_brother_project75_part1() {
    test_file("samples/brother/project75_part1.fcm");
}

#[test]
fn parses_brother_project75_part2() {
    test_file("samples/brother/project75_part2.fcm");
}

#[test]
fn parses_brother_project76_part1() {
    test_file("samples/brother/project76_part1.fcm");
}

#[test]
fn parses_brother_project77_part1() {
    test_file("samples/brother/project77_part1.fcm");
}

#[test]
fn parses_brother_project77_part2() {
    test_file("samples/brother/project77_part2.fcm");
}

#[test]
fn parses_brother_project78_part1() {
    test_file("samples/brother/project78_part1.fcm");
}

#[test]
fn parses_brother_project79_part1() {
    test_file("samples/brother/project79_part1.fcm");
}

#[test]
fn parses_brother_project79_part2() {
    test_file("samples/brother/project79_part2.fcm");
}

#[test]
fn parses_brother_project7_part1() {
    test_file("samples/brother/project7_part1.fcm");
}

#[test]
fn parses_brother_project7_part2() {
    test_file("samples/brother/project7_part2.fcm");
}

#[test]
fn parses_brother_project7_part3() {
    test_file("samples/brother/project7_part3.fcm");
}

#[test]
fn parses_brother_project7_part4() {
    test_file("samples/brother/project7_part4.fcm");
}

#[test]
fn parses_brother_project7_part5() {
    test_file("samples/brother/project7_part5.fcm");
}

#[test]
fn parses_brother_project7_part6() {
    test_file("samples/brother/project7_part6.fcm");
}

#[test]
fn parses_brother_project80_part1() {
    test_file("samples/brother/project80_part1.fcm");
}

#[test]
fn parses_brother_project80_part2() {
    test_file("samples/brother/project80_part2.fcm");
}

#[test]
fn parses_brother_project80_part3() {
    test_file("samples/brother/project80_part3.fcm");
}

#[test]
fn parses_brother_project81_part1() {
    test_file("samples/brother/project81_part1.fcm");
}

#[test]
fn parses_brother_project83_part1() {
    test_file("samples/brother/project83_part1.fcm");
}

#[test]
fn parses_brother_project83_part2() {
    test_file("samples/brother/project83_part2.fcm");
}

#[test]
fn parses_brother_project84_part1() {
    test_file("samples/brother/project84_part1.fcm");
}

#[test]
fn parses_brother_project84_part2() {
    test_file("samples/brother/project84_part2.fcm");
}

#[test]
fn parses_brother_project84_part3() {
    test_file("samples/brother/project84_part3.fcm");
}

#[test]
fn parses_brother_project84_part4() {
    test_file("samples/brother/project84_part4.fcm");
}

#[test]
fn parses_brother_project84_part5() {
    test_file("samples/brother/project84_part5.fcm");
}

#[test]
fn parses_brother_project84_part6() {
    test_file("samples/brother/project84_part6.fcm");
}

#[test]
fn parses_brother_project85_part1() {
    test_file("samples/brother/project85_part1.fcm");
}

#[test]
fn parses_brother_project86_part1() {
    test_file("samples/brother/project86_part1.fcm");
}

#[test]
fn parses_brother_project86_part2() {
    test_file("samples/brother/project86_part2.fcm");
}

#[test]
fn parses_brother_project86_part3() {
    test_file("samples/brother/project86_part3.fcm");
}

#[test]
fn parses_brother_project86_part4() {
    test_file("samples/brother/project86_part4.fcm");
}

#[test]
fn parses_brother_project86_part5() {
    test_file("samples/brother/project86_part5.fcm");
}

#[test]
fn parses_brother_project87_part1() {
    test_file("samples/brother/project87_part1.fcm");
}

#[test]
fn parses_brother_project88_part1() {
    test_file("samples/brother/project88_part1.fcm");
}

#[test]
fn parses_brother_project88_part2() {
    test_file("samples/brother/project88_part2.fcm");
}

#[test]
fn parses_brother_project88_part3() {
    test_file("samples/brother/project88_part3.fcm");
}

#[test]
fn parses_brother_project88_part4() {
    test_file("samples/brother/project88_part4.fcm");
}

#[test]
fn parses_brother_project89_part1() {
    test_file("samples/brother/project89_part1.fcm");
}

#[test]
fn parses_brother_project89_part2() {
    test_file("samples/brother/project89_part2.fcm");
}

#[test]
fn parses_brother_project8_part1() {
    test_file("samples/brother/project8_part1.fcm");
}

#[test]
fn parses_brother_project8_part2() {
    test_file("samples/brother/project8_part2.fcm");
}

#[test]
fn parses_brother_project8_part3() {
    test_file("samples/brother/project8_part3.fcm");
}

#[test]
fn parses_brother_project8_part4() {
    test_file("samples/brother/project8_part4.fcm");
}

#[test]
fn parses_brother_project8_part5() {
    test_file("samples/brother/project8_part5.fcm");
}

#[test]
fn parses_brother_project8_part6() {
    test_file("samples/brother/project8_part6.fcm");
}

#[test]
fn parses_brother_project8_part7() {
    test_file("samples/brother/project8_part7.fcm");
}

#[test]
fn parses_brother_project90_part1() {
    test_file("samples/brother/project90_part1.fcm");
}

#[test]
fn parses_brother_project90_part2() {
    test_file("samples/brother/project90_part2.fcm");
}

#[test]
fn parses_brother_project91_part1() {
    test_file("samples/brother/project91_part1.fcm");
}

#[test]
fn parses_brother_project91_part2() {
    test_file("samples/brother/project91_part2.fcm");
}

#[test]
fn parses_brother_project91_part3() {
    test_file("samples/brother/project91_part3.fcm");
}

#[test]
fn parses_brother_project91_part4() {
    test_file("samples/brother/project91_part4.fcm");
}

#[test]
fn parses_brother_project92_part1() {
    test_file("samples/brother/project92_part1.fcm");
}

#[test]
fn parses_brother_project92_part2() {
    test_file("samples/brother/project92_part2.fcm");
}

#[test]
fn parses_brother_project92_part3() {
    test_file("samples/brother/project92_part3.fcm");
}

#[test]
fn parses_brother_project92_part4() {
    test_file("samples/brother/project92_part4.fcm");
}

#[test]
fn parses_brother_project93_part1() {
    test_file("samples/brother/project93_part1.fcm");
}

#[test]
fn parses_brother_project93_part2() {
    test_file("samples/brother/project93_part2.fcm");
}

#[test]
fn parses_brother_project94_part1() {
    test_file("samples/brother/project94_part1.fcm");
}

#[test]
fn parses_brother_project94_part2() {
    test_file("samples/brother/project94_part2.fcm");
}

#[test]
fn parses_brother_project95_part1() {
    test_file("samples/brother/project95_part1.fcm");
}

#[test]
fn parses_brother_project95_part2() {
    test_file("samples/brother/project95_part2.fcm");
}

#[test]
fn parses_brother_project95_part3() {
    test_file("samples/brother/project95_part3.fcm");
}

#[test]
fn parses_brother_project95_part4() {
    test_file("samples/brother/project95_part4.fcm");
}

#[test]
fn parses_brother_project96_part1() {
    test_file("samples/brother/project96_part1.fcm");
}

#[test]
fn parses_brother_project96_part2() {
    test_file("samples/brother/project96_part2.fcm");
}

#[test]
fn parses_brother_project97_part1() {
    test_file("samples/brother/project97_part1.fcm");
}

#[test]
fn parses_brother_project97_part2() {
    test_file("samples/brother/project97_part2.fcm");
}

#[test]
fn parses_brother_project97_part3() {
    test_file("samples/brother/project97_part3.fcm");
}

#[test]
fn parses_brother_project98_part1() {
    test_file("samples/brother/project98_part1.fcm");
}

#[test]
fn parses_brother_project99_part1() {
    test_file("samples/brother/project99_part1.fcm");
}

#[test]
fn parses_brother_project99_part2() {
    test_file("samples/brother/project99_part2.fcm");
}

#[test]
fn parses_brother_project9_part1() {
    test_file("samples/brother/project9_part1.fcm");
}

#[test]
fn parses_brother_project9_part2() {
    test_file("samples/brother/project9_part2.fcm");
}

#[test]
fn parses_brother_project9_part3() {
    test_file("samples/brother/project9_part3.fcm");
}

#[test]
fn parses_brother_project9_part4() {
    test_file("samples/brother/project9_part4.fcm");
}

#[test]
fn parses_brother_project9_part5() {
    test_file("samples/brother/project9_part5.fcm");
}

#[test]
fn parses_brother_project9_part6() {
    test_file("samples/brother/project9_part6.fcm");
}

#[test]
fn parses_brother_project9_part7() {
    test_file("samples/brother/project9_part7.fcm");
}

#[test]
fn parses_scan_u000001() {
    test_file("samples/scan/U000001.fcm");
}

#[test]
fn parses_scan_u000014() {
    test_file("samples/scan/U000014.fcm");
}

#[test]
fn parses_scan_u000015() {
    test_file("samples/scan/U000015.fcm");
}

#[test]
fn parses_scan_u000016() {
    test_file("samples/scan/U000016.fcm");
}

#[test]
fn parses_scan_u000017() {
    test_file("samples/scan/U000017.fcm");
}

#[test]
fn parses_scan_u000018() {
    test_file("samples/scan/U000018.fcm");
}

#[test]
fn parses_scan_u000019() {
    test_file("samples/scan/U000019.fcm");
}

#[test]
fn parses_scan_u000241() {
    test_file("samples/scan/U000241.fcm");
}

#[test]
fn parses_scan_u000242() {
    test_file("samples/scan/U000242.fcm");
}

#[test]
fn parses_scan_u000254() {
    test_file("samples/scan/U000254.fcm");
}

#[test]
fn parses_scan_u000255() {
    test_file("samples/scan/U000255.fcm");
}

#[test]
fn parses_scan_u000256() {
    test_file("samples/scan/U000256.fcm");
}

#[test]
fn parses_test_23_february_new_print_to_cut_test() {
    test_file("samples/test/23 February New Print to Cut Test.fcm");
}

#[test]
fn parses_test_cut_10pt() {
    test_file("samples/test/cut_10pt.fcm");
}

#[test]
fn parses_test_cut_18pt() {
    test_file("samples/test/cut_18pt.fcm");
}

#[test]
fn parses_test_cut_19pt() {
    test_file("samples/test/cut_19pt.fcm");
}

#[test]
fn parses_test_cut_1pt() {
    test_file("samples/test/cut_1pt.fcm");
}

#[test]
fn parses_test_cut_20pt() {
    test_file("samples/test/cut_20pt.fcm");
}

#[test]
fn parses_test_cut_2pt() {
    test_file("samples/test/cut_2pt.fcm");
}

#[test]
fn parses_test_cut_3pt() {
    test_file("samples/test/cut_3pt.fcm");
}

#[test]
fn parses_test_cut() {
    test_file("samples/test/cut.fcm");
}

#[test]
fn parses_test_cut_open() {
    test_file("samples/test/cut_open.fcm");
}

#[test]
fn parses_test_cut_recreated() {
    test_file("samples/test/cut_recreated.fcm");
}

#[test]
fn parses_test_draw_10pt() {
    test_file("samples/test/draw_10pt.fcm");
}

#[test]
fn parses_test_draw_1pt() {
    test_file("samples/test/draw_1pt.fcm");
}

#[test]
fn parses_test_draw_20pt() {
    test_file("samples/test/draw_20pt.fcm");
}

#[test]
fn parses_test_draw_5pt() {
    test_file("samples/test/draw_5pt.fcm");
}

#[test]
fn parses_test_draw() {
    test_file("samples/test/draw.fcm");
}

#[test]
fn parses_test_draw_filled_noline() {
    test_file("samples/test/draw_filled_noline.fcm");
}

#[test]
fn parses_test_draw_noline() {
    test_file("samples/test/draw_noline.fcm");
}

#[test]
fn parses_test_draw_red() {
    test_file("samples/test/draw_red.fcm");
}

#[test]
fn parses_test_draw_thick() {
    test_file("samples/test/draw_thick.fcm");
}

#[test]
fn parses_test_example() {
    test_file("samples/test/example.fcm");
}

#[test]
fn parses_test_filled() {
    test_file("samples/test/filled.fcm");
}

#[test]
fn parses_test_imperial() {
    test_file("samples/test/imperial.fcm");
}

#[test]
fn parses_test_longboi() {
    test_file("samples/test/longboi.fcm");
}

#[test]
fn parses_test_open_path() {
    test_file("samples/test/open_path.fcm");
}

#[test]
fn parses_test_red() {
    test_file("samples/test/red.fcm");
}

#[test]
fn parses_test_rotated() {
    test_file("samples/test/rotated.fcm");
}

#[test]
fn parses_test_simple() {
    test_file("samples/test/simple.fcm");
}

#[test]
fn parses_test_square() {
    test_file("samples/test/square.fcm");
}

#[test]
fn parses_test_text() {
    test_file("samples/test/text.fcm");
}

#[test]
fn parses_test_thicc() {
    test_file("samples/test/thicc.fcm");
}

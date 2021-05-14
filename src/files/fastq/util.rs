
// Remove read number in illumina (first number in 1:N:0:2)
pub fn remove_illumina_read_number<'a>(illumina_name: & 'a mut Vec<u8>) -> & 'a mut Vec<u8> {
    illumina_name.remove(illumina_name.len()-6);
    illumina_name
}

// Remove read number in SRA format (end digit separated with a /, e.g. /1 or /2)
pub fn trim_sra<'a>(sra_name: & 'a mut Vec<u8>) -> & 'a mut Vec<u8> {
    sra_name.pop();
    sra_name.pop();
    sra_name
}


#[derive(Default)]
struct MaybePassport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

#[derive(Debug)]
struct ValidPassport {
    byr: u64,
    iyr: u64,
    eyr: u64,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}
impl Eq for ValidPassport {}
impl PartialEq for ValidPassport {
    fn eq(&self, other: &Self) -> bool {
        self.byr == other.byr
            && self.iyr == other.iyr
            && self.eyr == other.eyr
            && self.hgt == other.hgt
            && self.hcl == other.hcl
            && self.ecl == other.ecl
            && self.pid == other.pid
    }
}

// struct InvalidPassportError {

// };

fn to_passport(passport: MaybePassport) -> Result<ValidPassport, String> {
    Ok(ValidPassport {
        byr: passport
            .byr
            .ok_or("Birth Year missing")?
            .parse::<u64>()
            .or(Err("Birth Year invalid"))?,
        iyr: passport
            .iyr
            .ok_or("Issue Year missing")?
            .parse::<u64>()
            .or(Err("Issue Year invalid"))?,
        eyr: passport
            .eyr
            .ok_or("Expiration Year missing")?
            .parse::<u64>()
            .or(Err("Expiration Year invalid"))?,
        hgt: passport.hgt.ok_or("Height missing")?,
        hcl: passport.hcl.ok_or("Hair Color missing")?,
        ecl: passport.ecl.ok_or("Eye Color missing")?,
        pid: passport.pid.ok_or("Passport ID missing")?,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::util::read_file;

    #[test]
    fn check_maybe_to_passport_nones() {
        let result = to_passport(MaybePassport {
            ..Default::default()
        });
        assert_eq!(
            Err("Birth Year missing".to_owned()),
            result,
            "Should get an error if there is no birth year"
        );
    }
    #[test]
    fn check_maybe_to_passport_invalid_byr() {
        let result = to_passport(MaybePassport {
            byr: Some("not-a-number".to_owned()),
            ..Default::default()
        });
        assert_eq!(
            Err("Birth Year invalid".to_owned()),
            result,
            "Should get an error if birth year is not a number"
        )
    }
    #[test]
    fn check_maybe_to_passport_valid() {
        let result = to_passport(MaybePassport {
            ecl: Some("gry".to_owned()),
            pid: Some("860033327".to_owned()),
            eyr: Some("2020".to_owned()),
            hcl: Some("#fffffd".to_owned()),
            byr: Some("1937".to_owned()),
            iyr: Some("2017".to_owned()),
            cid: Some("147".to_owned()),
            hgt: Some("183cm".to_owned()),
            ..Default::default()
        });
        assert_eq!(
            Ok(ValidPassport {
                ecl: "gry".to_owned(),
                pid: "860033327".to_owned(),
                eyr: 2020,
                hcl: "#fffffd".to_owned(),
                byr: 1937,
                iyr: 2017,
                hgt: "183cm".to_owned(),
            }),
            result,
            "Should get an error if birth year is not a number"
        )
    }
}

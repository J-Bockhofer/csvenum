fn main() {
    
}

#[derive(Debug, Eq, PartialEq)]
pub enum Country
{
    /// `AFG` <br>
    /// country_code: usize = `4` <br>
    /// name: &str = `Afghanistan` <br>
    /// alpha_2: &str = `AF` <br>
    AFG,
    
    /// `ALA` <br>
    /// country_code: usize = `248` <br>
    /// name: &str = `Ãland Islands` <br>
    /// alpha_2: &str = `AX` <br>
    ALA,
    
    /// `ALB` <br>
    /// country_code: usize = `8` <br>
    /// name: &str = `Albania` <br>
    /// alpha_2: &str = `AL` <br>
    ALB,
    
    /// `DZA` <br>
    /// country_code: usize = `12` <br>
    /// name: &str = `Algeria` <br>
    /// alpha_2: &str = `DZ` <br>
    DZA,
    
    /// `ASM` <br>
    /// country_code: usize = `16` <br>
    /// name: &str = `American Samoa` <br>
    /// alpha_2: &str = `AS` <br>
    ASM,
    
    /// `AND` <br>
    /// country_code: usize = `20` <br>
    /// name: &str = `Andorra` <br>
    /// alpha_2: &str = `AD` <br>
    AND,
    
    /// `AGO` <br>
    /// country_code: usize = `24` <br>
    /// name: &str = `Angola` <br>
    /// alpha_2: &str = `AO` <br>
    AGO,
    
    /// `AIA` <br>
    /// country_code: usize = `660` <br>
    /// name: &str = `Anguilla` <br>
    /// alpha_2: &str = `AI` <br>
    AIA,
    
    /// `ATA` <br>
    /// country_code: usize = `10` <br>
    /// name: &str = `Antarctica` <br>
    /// alpha_2: &str = `AQ` <br>
    ATA,
    
    /// `ATG` <br>
    /// country_code: usize = `28` <br>
    /// name: &str = `Antigua and Barbuda` <br>
    /// alpha_2: &str = `AG` <br>
    ATG,
    
    /// `ARG` <br>
    /// country_code: usize = `32` <br>
    /// name: &str = `Argentina` <br>
    /// alpha_2: &str = `AR` <br>
    ARG,
    
    /// `ARM` <br>
    /// country_code: usize = `51` <br>
    /// name: &str = `Armenia` <br>
    /// alpha_2: &str = `AM` <br>
    ARM,
    
    /// `ABW` <br>
    /// country_code: usize = `533` <br>
    /// name: &str = `Aruba` <br>
    /// alpha_2: &str = `AW` <br>
    ABW,
    
    /// `AUS` <br>
    /// country_code: usize = `36` <br>
    /// name: &str = `Australia` <br>
    /// alpha_2: &str = `AU` <br>
    AUS,
    
    /// `AUT` <br>
    /// country_code: usize = `40` <br>
    /// name: &str = `Austria` <br>
    /// alpha_2: &str = `AT` <br>
    AUT,
    
    /// `AZE` <br>
    /// country_code: usize = `31` <br>
    /// name: &str = `Azerbaijan` <br>
    /// alpha_2: &str = `AZ` <br>
    AZE,
    
    /// `BHS` <br>
    /// country_code: usize = `44` <br>
    /// name: &str = `Bahamas` <br>
    /// alpha_2: &str = `BS` <br>
    BHS,
    
    /// `BHR` <br>
    /// country_code: usize = `48` <br>
    /// name: &str = `Bahrain` <br>
    /// alpha_2: &str = `BH` <br>
    BHR,
    
    /// `BGD` <br>
    /// country_code: usize = `50` <br>
    /// name: &str = `Bangladesh` <br>
    /// alpha_2: &str = `BD` <br>
    BGD,
    
    /// `BRB` <br>
    /// country_code: usize = `52` <br>
    /// name: &str = `Barbados` <br>
    /// alpha_2: &str = `BB` <br>
    BRB,
    
    /// `BLR` <br>
    /// country_code: usize = `112` <br>
    /// name: &str = `Belarus` <br>
    /// alpha_2: &str = `BY` <br>
    BLR,
    
    /// `BEL` <br>
    /// country_code: usize = `56` <br>
    /// name: &str = `Belgium` <br>
    /// alpha_2: &str = `BE` <br>
    BEL,
    
    /// `BLZ` <br>
    /// country_code: usize = `84` <br>
    /// name: &str = `Belize` <br>
    /// alpha_2: &str = `BZ` <br>
    BLZ,
    
    /// `BEN` <br>
    /// country_code: usize = `204` <br>
    /// name: &str = `Benin` <br>
    /// alpha_2: &str = `BJ` <br>
    BEN,
    
    /// `BMU` <br>
    /// country_code: usize = `60` <br>
    /// name: &str = `Bermuda` <br>
    /// alpha_2: &str = `BM` <br>
    BMU,
    
    /// `BTN` <br>
    /// country_code: usize = `64` <br>
    /// name: &str = `Bhutan` <br>
    /// alpha_2: &str = `BT` <br>
    BTN,
    
    /// `BOL` <br>
    /// country_code: usize = `68` <br>
    /// name: &str = `Bolivia (Plurinational State of)` <br>
    /// alpha_2: &str = `BO` <br>
    BOL,
    
    /// `BES` <br>
    /// country_code: usize = `535` <br>
    /// name: &str = `Bonaire, Sint Eustatius and Saba` <br>
    /// alpha_2: &str = `BQ` <br>
    BES,
    
    /// `BIH` <br>
    /// country_code: usize = `70` <br>
    /// name: &str = `Bosnia and Herzegovina` <br>
    /// alpha_2: &str = `BA` <br>
    BIH,
    
    /// `BWA` <br>
    /// country_code: usize = `72` <br>
    /// name: &str = `Botswana` <br>
    /// alpha_2: &str = `BW` <br>
    BWA,
    
    /// `BVT` <br>
    /// country_code: usize = `74` <br>
    /// name: &str = `Bouvet Island` <br>
    /// alpha_2: &str = `BV` <br>
    BVT,
    
    /// `BRA` <br>
    /// country_code: usize = `76` <br>
    /// name: &str = `Brazil` <br>
    /// alpha_2: &str = `BR` <br>
    BRA,
    
    /// `IOT` <br>
    /// country_code: usize = `86` <br>
    /// name: &str = `British Indian Ocean Territory` <br>
    /// alpha_2: &str = `IO` <br>
    IOT,
    
    /// `BRN` <br>
    /// country_code: usize = `96` <br>
    /// name: &str = `Brunei Darussalam` <br>
    /// alpha_2: &str = `BN` <br>
    BRN,
    
    /// `BGR` <br>
    /// country_code: usize = `100` <br>
    /// name: &str = `Bulgaria` <br>
    /// alpha_2: &str = `BG` <br>
    BGR,
    
    /// `BFA` <br>
    /// country_code: usize = `854` <br>
    /// name: &str = `Burkina Faso` <br>
    /// alpha_2: &str = `BF` <br>
    BFA,
    
    /// `BDI` <br>
    /// country_code: usize = `108` <br>
    /// name: &str = `Burundi` <br>
    /// alpha_2: &str = `BI` <br>
    BDI,
    
    /// `CPV` <br>
    /// country_code: usize = `132` <br>
    /// name: &str = `Cabo Verde` <br>
    /// alpha_2: &str = `CV` <br>
    CPV,
    
    /// `KHM` <br>
    /// country_code: usize = `116` <br>
    /// name: &str = `Cambodia` <br>
    /// alpha_2: &str = `KH` <br>
    KHM,
    
    /// `CMR` <br>
    /// country_code: usize = `120` <br>
    /// name: &str = `Cameroon` <br>
    /// alpha_2: &str = `CM` <br>
    CMR,
    
    /// `CAN` <br>
    /// country_code: usize = `124` <br>
    /// name: &str = `Canada` <br>
    /// alpha_2: &str = `CA` <br>
    CAN,
    
    /// `CYM` <br>
    /// country_code: usize = `136` <br>
    /// name: &str = `Cayman Islands` <br>
    /// alpha_2: &str = `KY` <br>
    CYM,
    
    /// `CAF` <br>
    /// country_code: usize = `140` <br>
    /// name: &str = `Central African Republic` <br>
    /// alpha_2: &str = `CF` <br>
    CAF,
    
    /// `TCD` <br>
    /// country_code: usize = `148` <br>
    /// name: &str = `Chad` <br>
    /// alpha_2: &str = `TD` <br>
    TCD,
    
    /// `CHL` <br>
    /// country_code: usize = `152` <br>
    /// name: &str = `Chile` <br>
    /// alpha_2: &str = `CL` <br>
    CHL,
    
    /// `CHN` <br>
    /// country_code: usize = `156` <br>
    /// name: &str = `China` <br>
    /// alpha_2: &str = `CN` <br>
    CHN,
    
    /// `CXR` <br>
    /// country_code: usize = `162` <br>
    /// name: &str = `Christmas Island` <br>
    /// alpha_2: &str = `CX` <br>
    CXR,
    
    /// `CCK` <br>
    /// country_code: usize = `166` <br>
    /// name: &str = `Cocos (Keeling) Islands` <br>
    /// alpha_2: &str = `CC` <br>
    CCK,
    
    /// `COL` <br>
    /// country_code: usize = `170` <br>
    /// name: &str = `Colombia` <br>
    /// alpha_2: &str = `CO` <br>
    COL,
    
    /// `COM` <br>
    /// country_code: usize = `174` <br>
    /// name: &str = `Comoros` <br>
    /// alpha_2: &str = `KM` <br>
    COM,
    
    /// `COG` <br>
    /// country_code: usize = `178` <br>
    /// name: &str = `Congo` <br>
    /// alpha_2: &str = `CG` <br>
    COG,
    
    /// `COD` <br>
    /// country_code: usize = `180` <br>
    /// name: &str = `Congo, Democratic Republic of the` <br>
    /// alpha_2: &str = `CD` <br>
    COD,
    
    /// `COK` <br>
    /// country_code: usize = `184` <br>
    /// name: &str = `Cook Islands` <br>
    /// alpha_2: &str = `CK` <br>
    COK,
    
    /// `CRI` <br>
    /// country_code: usize = `188` <br>
    /// name: &str = `Costa Rica` <br>
    /// alpha_2: &str = `CR` <br>
    CRI,
    
    /// `CIV` <br>
    /// country_code: usize = `384` <br>
    /// name: &str = `CÃ´te d'Ivoire` <br>
    /// alpha_2: &str = `CI` <br>
    CIV,
    
    /// `HRV` <br>
    /// country_code: usize = `191` <br>
    /// name: &str = `Croatia` <br>
    /// alpha_2: &str = `HR` <br>
    HRV,
    
    /// `CUB` <br>
    /// country_code: usize = `192` <br>
    /// name: &str = `Cuba` <br>
    /// alpha_2: &str = `CU` <br>
    CUB,
    
    /// `CUW` <br>
    /// country_code: usize = `531` <br>
    /// name: &str = `CuraÃ§ao` <br>
    /// alpha_2: &str = `CW` <br>
    CUW,
    
    /// `CYP` <br>
    /// country_code: usize = `196` <br>
    /// name: &str = `Cyprus` <br>
    /// alpha_2: &str = `CY` <br>
    CYP,
    
    /// `CZE` <br>
    /// country_code: usize = `203` <br>
    /// name: &str = `Czechia` <br>
    /// alpha_2: &str = `CZ` <br>
    CZE,
    
    /// `DNK` <br>
    /// country_code: usize = `208` <br>
    /// name: &str = `Denmark` <br>
    /// alpha_2: &str = `DK` <br>
    DNK,
    
    /// `DJI` <br>
    /// country_code: usize = `262` <br>
    /// name: &str = `Djibouti` <br>
    /// alpha_2: &str = `DJ` <br>
    DJI,
    
    /// `DMA` <br>
    /// country_code: usize = `212` <br>
    /// name: &str = `Dominica` <br>
    /// alpha_2: &str = `DM` <br>
    DMA,
    
    /// `DOM` <br>
    /// country_code: usize = `214` <br>
    /// name: &str = `Dominican Republic` <br>
    /// alpha_2: &str = `DO` <br>
    DOM,
    
    /// `ECU` <br>
    /// country_code: usize = `218` <br>
    /// name: &str = `Ecuador` <br>
    /// alpha_2: &str = `EC` <br>
    ECU,
    
    /// `EGY` <br>
    /// country_code: usize = `818` <br>
    /// name: &str = `Egypt` <br>
    /// alpha_2: &str = `EG` <br>
    EGY,
    
    /// `SLV` <br>
    /// country_code: usize = `222` <br>
    /// name: &str = `El Salvador` <br>
    /// alpha_2: &str = `SV` <br>
    SLV,
    
    /// `GNQ` <br>
    /// country_code: usize = `226` <br>
    /// name: &str = `Equatorial Guinea` <br>
    /// alpha_2: &str = `GQ` <br>
    GNQ,
    
    /// `ERI` <br>
    /// country_code: usize = `232` <br>
    /// name: &str = `Eritrea` <br>
    /// alpha_2: &str = `ER` <br>
    ERI,
    
    /// `EST` <br>
    /// country_code: usize = `233` <br>
    /// name: &str = `Estonia` <br>
    /// alpha_2: &str = `EE` <br>
    EST,
    
    /// `SWZ` <br>
    /// country_code: usize = `748` <br>
    /// name: &str = `Eswatini` <br>
    /// alpha_2: &str = `SZ` <br>
    SWZ,
    
    /// `ETH` <br>
    /// country_code: usize = `231` <br>
    /// name: &str = `Ethiopia` <br>
    /// alpha_2: &str = `ET` <br>
    ETH,
    
    /// `FLK` <br>
    /// country_code: usize = `238` <br>
    /// name: &str = `Falkland Islands (Malvinas)` <br>
    /// alpha_2: &str = `FK` <br>
    FLK,
    
    /// `FRO` <br>
    /// country_code: usize = `234` <br>
    /// name: &str = `Faroe Islands` <br>
    /// alpha_2: &str = `FO` <br>
    FRO,
    
    /// `FJI` <br>
    /// country_code: usize = `242` <br>
    /// name: &str = `Fiji` <br>
    /// alpha_2: &str = `FJ` <br>
    FJI,
    
    /// `FIN` <br>
    /// country_code: usize = `246` <br>
    /// name: &str = `Finland` <br>
    /// alpha_2: &str = `FI` <br>
    FIN,
    
    /// `FRA` <br>
    /// country_code: usize = `250` <br>
    /// name: &str = `France` <br>
    /// alpha_2: &str = `FR` <br>
    FRA,
    
    /// `GUF` <br>
    /// country_code: usize = `254` <br>
    /// name: &str = `French Guiana` <br>
    /// alpha_2: &str = `GF` <br>
    GUF,
    
    /// `PYF` <br>
    /// country_code: usize = `258` <br>
    /// name: &str = `French Polynesia` <br>
    /// alpha_2: &str = `PF` <br>
    PYF,
    
    /// `ATF` <br>
    /// country_code: usize = `260` <br>
    /// name: &str = `French Southern Territories` <br>
    /// alpha_2: &str = `TF` <br>
    ATF,
    
    /// `GAB` <br>
    /// country_code: usize = `266` <br>
    /// name: &str = `Gabon` <br>
    /// alpha_2: &str = `GA` <br>
    GAB,
    
    /// `GMB` <br>
    /// country_code: usize = `270` <br>
    /// name: &str = `Gambia` <br>
    /// alpha_2: &str = `GM` <br>
    GMB,
    
    /// `GEO` <br>
    /// country_code: usize = `268` <br>
    /// name: &str = `Georgia` <br>
    /// alpha_2: &str = `GE` <br>
    GEO,
    
    /// `DEU` <br>
    /// country_code: usize = `276` <br>
    /// name: &str = `Germany` <br>
    /// alpha_2: &str = `DE` <br>
    DEU,
    
    /// `GHA` <br>
    /// country_code: usize = `288` <br>
    /// name: &str = `Ghana` <br>
    /// alpha_2: &str = `GH` <br>
    GHA,
    
    /// `GIB` <br>
    /// country_code: usize = `292` <br>
    /// name: &str = `Gibraltar` <br>
    /// alpha_2: &str = `GI` <br>
    GIB,
    
    /// `GRC` <br>
    /// country_code: usize = `300` <br>
    /// name: &str = `Greece` <br>
    /// alpha_2: &str = `GR` <br>
    GRC,
    
    /// `GRL` <br>
    /// country_code: usize = `304` <br>
    /// name: &str = `Greenland` <br>
    /// alpha_2: &str = `GL` <br>
    GRL,
    
    /// `GRD` <br>
    /// country_code: usize = `308` <br>
    /// name: &str = `Grenada` <br>
    /// alpha_2: &str = `GD` <br>
    GRD,
    
    /// `GLP` <br>
    /// country_code: usize = `312` <br>
    /// name: &str = `Guadeloupe` <br>
    /// alpha_2: &str = `GP` <br>
    GLP,
    
    /// `GUM` <br>
    /// country_code: usize = `316` <br>
    /// name: &str = `Guam` <br>
    /// alpha_2: &str = `GU` <br>
    GUM,
    
    /// `GTM` <br>
    /// country_code: usize = `320` <br>
    /// name: &str = `Guatemala` <br>
    /// alpha_2: &str = `GT` <br>
    GTM,
    
    /// `GGY` <br>
    /// country_code: usize = `831` <br>
    /// name: &str = `Guernsey` <br>
    /// alpha_2: &str = `GG` <br>
    GGY,
    
    /// `GIN` <br>
    /// country_code: usize = `324` <br>
    /// name: &str = `Guinea` <br>
    /// alpha_2: &str = `GN` <br>
    GIN,
    
    /// `GNB` <br>
    /// country_code: usize = `624` <br>
    /// name: &str = `Guinea-Bissau` <br>
    /// alpha_2: &str = `GW` <br>
    GNB,
    
    /// `GUY` <br>
    /// country_code: usize = `328` <br>
    /// name: &str = `Guyana` <br>
    /// alpha_2: &str = `GY` <br>
    GUY,
    
    /// `HTI` <br>
    /// country_code: usize = `332` <br>
    /// name: &str = `Haiti` <br>
    /// alpha_2: &str = `HT` <br>
    HTI,
    
    /// `HMD` <br>
    /// country_code: usize = `334` <br>
    /// name: &str = `Heard Island and McDonald Islands` <br>
    /// alpha_2: &str = `HM` <br>
    HMD,
    
    /// `VAT` <br>
    /// country_code: usize = `336` <br>
    /// name: &str = `Holy See` <br>
    /// alpha_2: &str = `VA` <br>
    VAT,
    
    /// `HND` <br>
    /// country_code: usize = `340` <br>
    /// name: &str = `Honduras` <br>
    /// alpha_2: &str = `HN` <br>
    HND,
    
    /// `HKG` <br>
    /// country_code: usize = `344` <br>
    /// name: &str = `Hong Kong` <br>
    /// alpha_2: &str = `HK` <br>
    HKG,
    
    /// `HUN` <br>
    /// country_code: usize = `348` <br>
    /// name: &str = `Hungary` <br>
    /// alpha_2: &str = `HU` <br>
    HUN,
    
    /// `ISL` <br>
    /// country_code: usize = `352` <br>
    /// name: &str = `Iceland` <br>
    /// alpha_2: &str = `IS` <br>
    ISL,
    
    /// `IND` <br>
    /// country_code: usize = `356` <br>
    /// name: &str = `India` <br>
    /// alpha_2: &str = `IN` <br>
    IND,
    
    /// `IDN` <br>
    /// country_code: usize = `360` <br>
    /// name: &str = `Indonesia` <br>
    /// alpha_2: &str = `ID` <br>
    IDN,
    
    /// `IRN` <br>
    /// country_code: usize = `364` <br>
    /// name: &str = `Iran (Islamic Republic of)` <br>
    /// alpha_2: &str = `IR` <br>
    IRN,
    
    /// `IRQ` <br>
    /// country_code: usize = `368` <br>
    /// name: &str = `Iraq` <br>
    /// alpha_2: &str = `IQ` <br>
    IRQ,
    
    /// `IRL` <br>
    /// country_code: usize = `372` <br>
    /// name: &str = `Ireland` <br>
    /// alpha_2: &str = `IE` <br>
    IRL,
    
    /// `IMN` <br>
    /// country_code: usize = `833` <br>
    /// name: &str = `Isle of Man` <br>
    /// alpha_2: &str = `IM` <br>
    IMN,
    
    /// `ISR` <br>
    /// country_code: usize = `376` <br>
    /// name: &str = `Israel` <br>
    /// alpha_2: &str = `IL` <br>
    ISR,
    
    /// `ITA` <br>
    /// country_code: usize = `380` <br>
    /// name: &str = `Italy` <br>
    /// alpha_2: &str = `IT` <br>
    ITA,
    
    /// `JAM` <br>
    /// country_code: usize = `388` <br>
    /// name: &str = `Jamaica` <br>
    /// alpha_2: &str = `JM` <br>
    JAM,
    
    /// `JPN` <br>
    /// country_code: usize = `392` <br>
    /// name: &str = `Japan` <br>
    /// alpha_2: &str = `JP` <br>
    JPN,
    
    /// `JEY` <br>
    /// country_code: usize = `832` <br>
    /// name: &str = `Jersey` <br>
    /// alpha_2: &str = `JE` <br>
    JEY,
    
    /// `JOR` <br>
    /// country_code: usize = `400` <br>
    /// name: &str = `Jordan` <br>
    /// alpha_2: &str = `JO` <br>
    JOR,
    
    /// `KAZ` <br>
    /// country_code: usize = `398` <br>
    /// name: &str = `Kazakhstan` <br>
    /// alpha_2: &str = `KZ` <br>
    KAZ,
    
    /// `KEN` <br>
    /// country_code: usize = `404` <br>
    /// name: &str = `Kenya` <br>
    /// alpha_2: &str = `KE` <br>
    KEN,
    
    /// `KIR` <br>
    /// country_code: usize = `296` <br>
    /// name: &str = `Kiribati` <br>
    /// alpha_2: &str = `KI` <br>
    KIR,
    
    /// `PRK` <br>
    /// country_code: usize = `408` <br>
    /// name: &str = `Korea (Democratic People's Republic of)` <br>
    /// alpha_2: &str = `KP` <br>
    PRK,
    
    /// `KOR` <br>
    /// country_code: usize = `410` <br>
    /// name: &str = `Korea, Republic of` <br>
    /// alpha_2: &str = `KR` <br>
    KOR,
    
    /// `KWT` <br>
    /// country_code: usize = `414` <br>
    /// name: &str = `Kuwait` <br>
    /// alpha_2: &str = `KW` <br>
    KWT,
    
    /// `KGZ` <br>
    /// country_code: usize = `417` <br>
    /// name: &str = `Kyrgyzstan` <br>
    /// alpha_2: &str = `KG` <br>
    KGZ,
    
    /// `LAO` <br>
    /// country_code: usize = `418` <br>
    /// name: &str = `Lao People's Democratic Republic` <br>
    /// alpha_2: &str = `LA` <br>
    LAO,
    
    /// `LVA` <br>
    /// country_code: usize = `428` <br>
    /// name: &str = `Latvia` <br>
    /// alpha_2: &str = `LV` <br>
    LVA,
    
    /// `LBN` <br>
    /// country_code: usize = `422` <br>
    /// name: &str = `Lebanon` <br>
    /// alpha_2: &str = `LB` <br>
    LBN,
    
    /// `LSO` <br>
    /// country_code: usize = `426` <br>
    /// name: &str = `Lesotho` <br>
    /// alpha_2: &str = `LS` <br>
    LSO,
    
    /// `LBR` <br>
    /// country_code: usize = `430` <br>
    /// name: &str = `Liberia` <br>
    /// alpha_2: &str = `LR` <br>
    LBR,
    
    /// `LBY` <br>
    /// country_code: usize = `434` <br>
    /// name: &str = `Libya` <br>
    /// alpha_2: &str = `LY` <br>
    LBY,
    
    /// `LIE` <br>
    /// country_code: usize = `438` <br>
    /// name: &str = `Liechtenstein` <br>
    /// alpha_2: &str = `LI` <br>
    LIE,
    
    /// `LTU` <br>
    /// country_code: usize = `440` <br>
    /// name: &str = `Lithuania` <br>
    /// alpha_2: &str = `LT` <br>
    LTU,
    
    /// `LUX` <br>
    /// country_code: usize = `442` <br>
    /// name: &str = `Luxembourg` <br>
    /// alpha_2: &str = `LU` <br>
    LUX,
    
    /// `MAC` <br>
    /// country_code: usize = `446` <br>
    /// name: &str = `Macao` <br>
    /// alpha_2: &str = `MO` <br>
    MAC,
    
    /// `MDG` <br>
    /// country_code: usize = `450` <br>
    /// name: &str = `Madagascar` <br>
    /// alpha_2: &str = `MG` <br>
    MDG,
    
    /// `MWI` <br>
    /// country_code: usize = `454` <br>
    /// name: &str = `Malawi` <br>
    /// alpha_2: &str = `MW` <br>
    MWI,
    
    /// `MYS` <br>
    /// country_code: usize = `458` <br>
    /// name: &str = `Malaysia` <br>
    /// alpha_2: &str = `MY` <br>
    MYS,
    
    /// `MDV` <br>
    /// country_code: usize = `462` <br>
    /// name: &str = `Maldives` <br>
    /// alpha_2: &str = `MV` <br>
    MDV,
    
    /// `MLI` <br>
    /// country_code: usize = `466` <br>
    /// name: &str = `Mali` <br>
    /// alpha_2: &str = `ML` <br>
    MLI,
    
    /// `MLT` <br>
    /// country_code: usize = `470` <br>
    /// name: &str = `Malta` <br>
    /// alpha_2: &str = `MT` <br>
    MLT,
    
    /// `MHL` <br>
    /// country_code: usize = `584` <br>
    /// name: &str = `Marshall Islands` <br>
    /// alpha_2: &str = `MH` <br>
    MHL,
    
    /// `MTQ` <br>
    /// country_code: usize = `474` <br>
    /// name: &str = `Martinique` <br>
    /// alpha_2: &str = `MQ` <br>
    MTQ,
    
    /// `MRT` <br>
    /// country_code: usize = `478` <br>
    /// name: &str = `Mauritania` <br>
    /// alpha_2: &str = `MR` <br>
    MRT,
    
    /// `MUS` <br>
    /// country_code: usize = `480` <br>
    /// name: &str = `Mauritius` <br>
    /// alpha_2: &str = `MU` <br>
    MUS,
    
    /// `MYT` <br>
    /// country_code: usize = `175` <br>
    /// name: &str = `Mayotte` <br>
    /// alpha_2: &str = `YT` <br>
    MYT,
    
    /// `MEX` <br>
    /// country_code: usize = `484` <br>
    /// name: &str = `Mexico` <br>
    /// alpha_2: &str = `MX` <br>
    MEX,
    
    /// `FSM` <br>
    /// country_code: usize = `583` <br>
    /// name: &str = `Micronesia (Federated States of)` <br>
    /// alpha_2: &str = `FM` <br>
    FSM,
    
    /// `MDA` <br>
    /// country_code: usize = `498` <br>
    /// name: &str = `Moldova, Republic of` <br>
    /// alpha_2: &str = `MD` <br>
    MDA,
    
    /// `MCO` <br>
    /// country_code: usize = `492` <br>
    /// name: &str = `Monaco` <br>
    /// alpha_2: &str = `MC` <br>
    MCO,
    
    /// `MNG` <br>
    /// country_code: usize = `496` <br>
    /// name: &str = `Mongolia` <br>
    /// alpha_2: &str = `MN` <br>
    MNG,
    
    /// `MNE` <br>
    /// country_code: usize = `499` <br>
    /// name: &str = `Montenegro` <br>
    /// alpha_2: &str = `ME` <br>
    MNE,
    
    /// `MSR` <br>
    /// country_code: usize = `500` <br>
    /// name: &str = `Montserrat` <br>
    /// alpha_2: &str = `MS` <br>
    MSR,
    
    /// `MAR` <br>
    /// country_code: usize = `504` <br>
    /// name: &str = `Morocco` <br>
    /// alpha_2: &str = `MA` <br>
    MAR,
    
    /// `MOZ` <br>
    /// country_code: usize = `508` <br>
    /// name: &str = `Mozambique` <br>
    /// alpha_2: &str = `MZ` <br>
    MOZ,
    
    /// `MMR` <br>
    /// country_code: usize = `104` <br>
    /// name: &str = `Myanmar` <br>
    /// alpha_2: &str = `MM` <br>
    MMR,
    
    /// `NAM` <br>
    /// country_code: usize = `516` <br>
    /// name: &str = `Namibia` <br>
    /// alpha_2: &str = `NA` <br>
    NAM,
    
    /// `NRU` <br>
    /// country_code: usize = `520` <br>
    /// name: &str = `Nauru` <br>
    /// alpha_2: &str = `NR` <br>
    NRU,
    
    /// `NPL` <br>
    /// country_code: usize = `524` <br>
    /// name: &str = `Nepal` <br>
    /// alpha_2: &str = `NP` <br>
    NPL,
    
    /// `NLD` <br>
    /// country_code: usize = `528` <br>
    /// name: &str = `Netherlands` <br>
    /// alpha_2: &str = `NL` <br>
    NLD,
    
    /// `NCL` <br>
    /// country_code: usize = `540` <br>
    /// name: &str = `New Caledonia` <br>
    /// alpha_2: &str = `NC` <br>
    NCL,
    
    /// `NZL` <br>
    /// country_code: usize = `554` <br>
    /// name: &str = `New Zealand` <br>
    /// alpha_2: &str = `NZ` <br>
    NZL,
    
    /// `NIC` <br>
    /// country_code: usize = `558` <br>
    /// name: &str = `Nicaragua` <br>
    /// alpha_2: &str = `NI` <br>
    NIC,
    
    /// `NER` <br>
    /// country_code: usize = `562` <br>
    /// name: &str = `Niger` <br>
    /// alpha_2: &str = `NE` <br>
    NER,
    
    /// `NGA` <br>
    /// country_code: usize = `566` <br>
    /// name: &str = `Nigeria` <br>
    /// alpha_2: &str = `NG` <br>
    NGA,
    
    /// `NIU` <br>
    /// country_code: usize = `570` <br>
    /// name: &str = `Niue` <br>
    /// alpha_2: &str = `NU` <br>
    NIU,
    
    /// `NFK` <br>
    /// country_code: usize = `574` <br>
    /// name: &str = `Norfolk Island` <br>
    /// alpha_2: &str = `NF` <br>
    NFK,
    
    /// `MKD` <br>
    /// country_code: usize = `807` <br>
    /// name: &str = `North Macedonia` <br>
    /// alpha_2: &str = `MK` <br>
    MKD,
    
    /// `MNP` <br>
    /// country_code: usize = `580` <br>
    /// name: &str = `Northern Mariana Islands` <br>
    /// alpha_2: &str = `MP` <br>
    MNP,
    
    /// `NOR` <br>
    /// country_code: usize = `578` <br>
    /// name: &str = `Norway` <br>
    /// alpha_2: &str = `NO` <br>
    NOR,
    
    /// `OMN` <br>
    /// country_code: usize = `512` <br>
    /// name: &str = `Oman` <br>
    /// alpha_2: &str = `OM` <br>
    OMN,
    
    /// `PAK` <br>
    /// country_code: usize = `586` <br>
    /// name: &str = `Pakistan` <br>
    /// alpha_2: &str = `PK` <br>
    PAK,
    
    /// `PLW` <br>
    /// country_code: usize = `585` <br>
    /// name: &str = `Palau` <br>
    /// alpha_2: &str = `PW` <br>
    PLW,
    
    /// `PSE` <br>
    /// country_code: usize = `275` <br>
    /// name: &str = `Palestine, State of` <br>
    /// alpha_2: &str = `PS` <br>
    PSE,
    
    /// `PAN` <br>
    /// country_code: usize = `591` <br>
    /// name: &str = `Panama` <br>
    /// alpha_2: &str = `PA` <br>
    PAN,
    
    /// `PNG` <br>
    /// country_code: usize = `598` <br>
    /// name: &str = `Papua New Guinea` <br>
    /// alpha_2: &str = `PG` <br>
    PNG,
    
    /// `PRY` <br>
    /// country_code: usize = `600` <br>
    /// name: &str = `Paraguay` <br>
    /// alpha_2: &str = `PY` <br>
    PRY,
    
    /// `PER` <br>
    /// country_code: usize = `604` <br>
    /// name: &str = `Peru` <br>
    /// alpha_2: &str = `PE` <br>
    PER,
    
    /// `PHL` <br>
    /// country_code: usize = `608` <br>
    /// name: &str = `Philippines` <br>
    /// alpha_2: &str = `PH` <br>
    PHL,
    
    /// `PCN` <br>
    /// country_code: usize = `612` <br>
    /// name: &str = `Pitcairn` <br>
    /// alpha_2: &str = `PN` <br>
    PCN,
    
    /// `POL` <br>
    /// country_code: usize = `616` <br>
    /// name: &str = `Poland` <br>
    /// alpha_2: &str = `PL` <br>
    POL,
    
    /// `PRT` <br>
    /// country_code: usize = `620` <br>
    /// name: &str = `Portugal` <br>
    /// alpha_2: &str = `PT` <br>
    PRT,
    
    /// `PRI` <br>
    /// country_code: usize = `630` <br>
    /// name: &str = `Puerto Rico` <br>
    /// alpha_2: &str = `PR` <br>
    PRI,
    
    /// `QAT` <br>
    /// country_code: usize = `634` <br>
    /// name: &str = `Qatar` <br>
    /// alpha_2: &str = `QA` <br>
    QAT,
    
    /// `REU` <br>
    /// country_code: usize = `638` <br>
    /// name: &str = `RÃ©union` <br>
    /// alpha_2: &str = `RE` <br>
    REU,
    
    /// `ROU` <br>
    /// country_code: usize = `642` <br>
    /// name: &str = `Romania` <br>
    /// alpha_2: &str = `RO` <br>
    ROU,
    
    /// `RUS` <br>
    /// country_code: usize = `643` <br>
    /// name: &str = `Russian Federation` <br>
    /// alpha_2: &str = `RU` <br>
    RUS,
    
    /// `RWA` <br>
    /// country_code: usize = `646` <br>
    /// name: &str = `Rwanda` <br>
    /// alpha_2: &str = `RW` <br>
    RWA,
    
    /// `BLM` <br>
    /// country_code: usize = `652` <br>
    /// name: &str = `Saint BarthÃ©lemy` <br>
    /// alpha_2: &str = `BL` <br>
    BLM,
    
    /// `SHN` <br>
    /// country_code: usize = `654` <br>
    /// name: &str = `Saint Helena, Ascension and Tristan da Cunha` <br>
    /// alpha_2: &str = `SH` <br>
    SHN,
    
    /// `KNA` <br>
    /// country_code: usize = `659` <br>
    /// name: &str = `Saint Kitts and Nevis` <br>
    /// alpha_2: &str = `KN` <br>
    KNA,
    
    /// `LCA` <br>
    /// country_code: usize = `662` <br>
    /// name: &str = `Saint Lucia` <br>
    /// alpha_2: &str = `LC` <br>
    LCA,
    
    /// `MAF` <br>
    /// country_code: usize = `663` <br>
    /// name: &str = `Saint Martin (French part)` <br>
    /// alpha_2: &str = `MF` <br>
    MAF,
    
    /// `SPM` <br>
    /// country_code: usize = `666` <br>
    /// name: &str = `Saint Pierre and Miquelon` <br>
    /// alpha_2: &str = `PM` <br>
    SPM,
    
    /// `VCT` <br>
    /// country_code: usize = `670` <br>
    /// name: &str = `Saint Vincent and the Grenadines` <br>
    /// alpha_2: &str = `VC` <br>
    VCT,
    
    /// `WSM` <br>
    /// country_code: usize = `882` <br>
    /// name: &str = `Samoa` <br>
    /// alpha_2: &str = `WS` <br>
    WSM,
    
    /// `SMR` <br>
    /// country_code: usize = `674` <br>
    /// name: &str = `San Marino` <br>
    /// alpha_2: &str = `SM` <br>
    SMR,
    
    /// `STP` <br>
    /// country_code: usize = `678` <br>
    /// name: &str = `Sao Tome and Principe` <br>
    /// alpha_2: &str = `ST` <br>
    STP,
    
    /// `SAU` <br>
    /// country_code: usize = `682` <br>
    /// name: &str = `Saudi Arabia` <br>
    /// alpha_2: &str = `SA` <br>
    SAU,
    
    /// `SEN` <br>
    /// country_code: usize = `686` <br>
    /// name: &str = `Senegal` <br>
    /// alpha_2: &str = `SN` <br>
    SEN,
    
    /// `SRB` <br>
    /// country_code: usize = `688` <br>
    /// name: &str = `Serbia` <br>
    /// alpha_2: &str = `RS` <br>
    SRB,
    
    /// `SYC` <br>
    /// country_code: usize = `690` <br>
    /// name: &str = `Seychelles` <br>
    /// alpha_2: &str = `SC` <br>
    SYC,
    
    /// `SLE` <br>
    /// country_code: usize = `694` <br>
    /// name: &str = `Sierra Leone` <br>
    /// alpha_2: &str = `SL` <br>
    SLE,
    
    /// `SGP` <br>
    /// country_code: usize = `702` <br>
    /// name: &str = `Singapore` <br>
    /// alpha_2: &str = `SG` <br>
    SGP,
    
    /// `SXM` <br>
    /// country_code: usize = `534` <br>
    /// name: &str = `Sint Maarten (Dutch part)` <br>
    /// alpha_2: &str = `SX` <br>
    SXM,
    
    /// `SVK` <br>
    /// country_code: usize = `703` <br>
    /// name: &str = `Slovakia` <br>
    /// alpha_2: &str = `SK` <br>
    SVK,
    
    /// `SVN` <br>
    /// country_code: usize = `705` <br>
    /// name: &str = `Slovenia` <br>
    /// alpha_2: &str = `SI` <br>
    SVN,
    
    /// `SLB` <br>
    /// country_code: usize = `90` <br>
    /// name: &str = `Solomon Islands` <br>
    /// alpha_2: &str = `SB` <br>
    SLB,
    
    /// `SOM` <br>
    /// country_code: usize = `706` <br>
    /// name: &str = `Somalia` <br>
    /// alpha_2: &str = `SO` <br>
    SOM,
    
    /// `ZAF` <br>
    /// country_code: usize = `710` <br>
    /// name: &str = `South Africa` <br>
    /// alpha_2: &str = `ZA` <br>
    ZAF,
    
    /// `SGS` <br>
    /// country_code: usize = `239` <br>
    /// name: &str = `South Georgia and the South Sandwich Islands` <br>
    /// alpha_2: &str = `GS` <br>
    SGS,
    
    /// `SSD` <br>
    /// country_code: usize = `728` <br>
    /// name: &str = `South Sudan` <br>
    /// alpha_2: &str = `SS` <br>
    SSD,
    
    /// `ESP` <br>
    /// country_code: usize = `724` <br>
    /// name: &str = `Spain` <br>
    /// alpha_2: &str = `ES` <br>
    ESP,
    
    /// `LKA` <br>
    /// country_code: usize = `144` <br>
    /// name: &str = `Sri Lanka` <br>
    /// alpha_2: &str = `LK` <br>
    LKA,
    
    /// `SDN` <br>
    /// country_code: usize = `729` <br>
    /// name: &str = `Sudan` <br>
    /// alpha_2: &str = `SD` <br>
    SDN,
    
    /// `SUR` <br>
    /// country_code: usize = `740` <br>
    /// name: &str = `Suriname` <br>
    /// alpha_2: &str = `SR` <br>
    SUR,
    
    /// `SJM` <br>
    /// country_code: usize = `744` <br>
    /// name: &str = `Svalbard and Jan Mayen` <br>
    /// alpha_2: &str = `SJ` <br>
    SJM,
    
    /// `SWE` <br>
    /// country_code: usize = `752` <br>
    /// name: &str = `Sweden` <br>
    /// alpha_2: &str = `SE` <br>
    SWE,
    
    /// `CHE` <br>
    /// country_code: usize = `756` <br>
    /// name: &str = `Switzerland` <br>
    /// alpha_2: &str = `CH` <br>
    CHE,
    
    /// `SYR` <br>
    /// country_code: usize = `760` <br>
    /// name: &str = `Syrian Arab Republic` <br>
    /// alpha_2: &str = `SY` <br>
    SYR,
    
    /// `TWN` <br>
    /// country_code: usize = `158` <br>
    /// name: &str = `Taiwan, Province of China` <br>
    /// alpha_2: &str = `TW` <br>
    TWN,
    
    /// `TJK` <br>
    /// country_code: usize = `762` <br>
    /// name: &str = `Tajikistan` <br>
    /// alpha_2: &str = `TJ` <br>
    TJK,
    
    /// `TZA` <br>
    /// country_code: usize = `834` <br>
    /// name: &str = `Tanzania, United Republic of` <br>
    /// alpha_2: &str = `TZ` <br>
    TZA,
    
    /// `THA` <br>
    /// country_code: usize = `764` <br>
    /// name: &str = `Thailand` <br>
    /// alpha_2: &str = `TH` <br>
    THA,
    
    /// `TLS` <br>
    /// country_code: usize = `626` <br>
    /// name: &str = `Timor-Leste` <br>
    /// alpha_2: &str = `TL` <br>
    TLS,
    
    /// `TGO` <br>
    /// country_code: usize = `768` <br>
    /// name: &str = `Togo` <br>
    /// alpha_2: &str = `TG` <br>
    TGO,
    
    /// `TKL` <br>
    /// country_code: usize = `772` <br>
    /// name: &str = `Tokelau` <br>
    /// alpha_2: &str = `TK` <br>
    TKL,
    
    /// `TON` <br>
    /// country_code: usize = `776` <br>
    /// name: &str = `Tonga` <br>
    /// alpha_2: &str = `TO` <br>
    TON,
    
    /// `TTO` <br>
    /// country_code: usize = `780` <br>
    /// name: &str = `Trinidad and Tobago` <br>
    /// alpha_2: &str = `TT` <br>
    TTO,
    
    /// `TUN` <br>
    /// country_code: usize = `788` <br>
    /// name: &str = `Tunisia` <br>
    /// alpha_2: &str = `TN` <br>
    TUN,
    
    /// `TUR` <br>
    /// country_code: usize = `792` <br>
    /// name: &str = `Turkey` <br>
    /// alpha_2: &str = `TR` <br>
    TUR,
    
    /// `TKM` <br>
    /// country_code: usize = `795` <br>
    /// name: &str = `Turkmenistan` <br>
    /// alpha_2: &str = `TM` <br>
    TKM,
    
    /// `TCA` <br>
    /// country_code: usize = `796` <br>
    /// name: &str = `Turks and Caicos Islands` <br>
    /// alpha_2: &str = `TC` <br>
    TCA,
    
    /// `TUV` <br>
    /// country_code: usize = `798` <br>
    /// name: &str = `Tuvalu` <br>
    /// alpha_2: &str = `TV` <br>
    TUV,
    
    /// `UGA` <br>
    /// country_code: usize = `800` <br>
    /// name: &str = `Uganda` <br>
    /// alpha_2: &str = `UG` <br>
    UGA,
    
    /// `UKR` <br>
    /// country_code: usize = `804` <br>
    /// name: &str = `Ukraine` <br>
    /// alpha_2: &str = `UA` <br>
    UKR,
    
    /// `ARE` <br>
    /// country_code: usize = `784` <br>
    /// name: &str = `United Arab Emirates` <br>
    /// alpha_2: &str = `AE` <br>
    ARE,
    
    /// `GBR` <br>
    /// country_code: usize = `826` <br>
    /// name: &str = `United Kingdom of Great Britain and Northern Ireland` <br>
    /// alpha_2: &str = `GB` <br>
    GBR,
    
    /// `USA` <br>
    /// country_code: usize = `840` <br>
    /// name: &str = `United States of America` <br>
    /// alpha_2: &str = `US` <br>
    USA,
    
    /// `UMI` <br>
    /// country_code: usize = `581` <br>
    /// name: &str = `United States Minor Outlying Islands` <br>
    /// alpha_2: &str = `UM` <br>
    UMI,
    
    /// `URY` <br>
    /// country_code: usize = `858` <br>
    /// name: &str = `Uruguay` <br>
    /// alpha_2: &str = `UY` <br>
    URY,
    
    /// `UZB` <br>
    /// country_code: usize = `860` <br>
    /// name: &str = `Uzbekistan` <br>
    /// alpha_2: &str = `UZ` <br>
    UZB,
    
    /// `VUT` <br>
    /// country_code: usize = `548` <br>
    /// name: &str = `Vanuatu` <br>
    /// alpha_2: &str = `VU` <br>
    VUT,
    
    /// `VEN` <br>
    /// country_code: usize = `862` <br>
    /// name: &str = `Venezuela (Bolivarian Republic of)` <br>
    /// alpha_2: &str = `VE` <br>
    VEN,
    
    /// `VNM` <br>
    /// country_code: usize = `704` <br>
    /// name: &str = `Viet Nam` <br>
    /// alpha_2: &str = `VN` <br>
    VNM,
    
    /// `VGB` <br>
    /// country_code: usize = `92` <br>
    /// name: &str = `Virgin Islands (British)` <br>
    /// alpha_2: &str = `VG` <br>
    VGB,
    
    /// `VIR` <br>
    /// country_code: usize = `850` <br>
    /// name: &str = `Virgin Islands (U.S.)` <br>
    /// alpha_2: &str = `VI` <br>
    VIR,
    
    /// `WLF` <br>
    /// country_code: usize = `876` <br>
    /// name: &str = `Wallis and Futuna` <br>
    /// alpha_2: &str = `WF` <br>
    WLF,
    
    /// `ESH` <br>
    /// country_code: usize = `732` <br>
    /// name: &str = `Western Sahara` <br>
    /// alpha_2: &str = `EH` <br>
    ESH,
    
    /// `YEM` <br>
    /// country_code: usize = `887` <br>
    /// name: &str = `Yemen` <br>
    /// alpha_2: &str = `YE` <br>
    YEM,
    
    /// `ZMB` <br>
    /// country_code: usize = `894` <br>
    /// name: &str = `Zambia` <br>
    /// alpha_2: &str = `ZM` <br>
    ZMB,
    
    /// `ZWE` <br>
    /// country_code: usize = `716` <br>
    /// name: &str = `Zimbabwe` <br>
    /// alpha_2: &str = `ZW` <br>
    ZWE,
    
}
impl Country
{
    pub fn get_all_variants() -> [Self; 249]
    {
        country_get_all_variants()
    }
    pub fn as_variant_str(&self) -> &'static str
    {
        country_as_variant_str(self)
    }
    pub fn from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Self>
    {
        country_from_variant_str(variantstr)
    }
    /// Function to convert from Country to country_code
    pub fn as_country_code(&self) -> usize
    {
        country_as_country_code(self)
    }
    pub fn from_country_code(country_code: usize) -> Option<Self>
    {
        country_from_country_code(country_code)
    }
    /// Function to convert from Country to name
    pub fn as_name(&self) -> &'static str
    {
        country_as_name(self)
    }
    pub fn from_name(name: &str) -> Option<Self>
    {
        country_from_name(name)
    }
    /// Function to convert from Country to alpha_2
    pub fn as_alpha_2(&self) -> &'static str
    {
        country_as_alpha_2(self)
    }
    pub fn from_alpha_2(alpha_2: &str) -> Option<Self>
    {
        country_from_alpha_2(alpha_2)
    }
}
impl std::fmt::Display for Country
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            Self::AFG => writeln!(f, "{}, country_code = 4 , name = Afghanistan , alpha_2 = AF ", self.as_variant_str())?,
            Self::ALA => writeln!(f, "{}, country_code = 248 , name = Ãland Islands , alpha_2 = AX ", self.as_variant_str())?,
            Self::ALB => writeln!(f, "{}, country_code = 8 , name = Albania , alpha_2 = AL ", self.as_variant_str())?,
            Self::DZA => writeln!(f, "{}, country_code = 12 , name = Algeria , alpha_2 = DZ ", self.as_variant_str())?,
            Self::ASM => writeln!(f, "{}, country_code = 16 , name = American Samoa , alpha_2 = AS ", self.as_variant_str())?,
            Self::AND => writeln!(f, "{}, country_code = 20 , name = Andorra , alpha_2 = AD ", self.as_variant_str())?,
            Self::AGO => writeln!(f, "{}, country_code = 24 , name = Angola , alpha_2 = AO ", self.as_variant_str())?,
            Self::AIA => writeln!(f, "{}, country_code = 660 , name = Anguilla , alpha_2 = AI ", self.as_variant_str())?,
            Self::ATA => writeln!(f, "{}, country_code = 10 , name = Antarctica , alpha_2 = AQ ", self.as_variant_str())?,
            Self::ATG => writeln!(f, "{}, country_code = 28 , name = Antigua and Barbuda , alpha_2 = AG ", self.as_variant_str())?,
            Self::ARG => writeln!(f, "{}, country_code = 32 , name = Argentina , alpha_2 = AR ", self.as_variant_str())?,
            Self::ARM => writeln!(f, "{}, country_code = 51 , name = Armenia , alpha_2 = AM ", self.as_variant_str())?,
            Self::ABW => writeln!(f, "{}, country_code = 533 , name = Aruba , alpha_2 = AW ", self.as_variant_str())?,
            Self::AUS => writeln!(f, "{}, country_code = 36 , name = Australia , alpha_2 = AU ", self.as_variant_str())?,
            Self::AUT => writeln!(f, "{}, country_code = 40 , name = Austria , alpha_2 = AT ", self.as_variant_str())?,
            Self::AZE => writeln!(f, "{}, country_code = 31 , name = Azerbaijan , alpha_2 = AZ ", self.as_variant_str())?,
            Self::BHS => writeln!(f, "{}, country_code = 44 , name = Bahamas , alpha_2 = BS ", self.as_variant_str())?,
            Self::BHR => writeln!(f, "{}, country_code = 48 , name = Bahrain , alpha_2 = BH ", self.as_variant_str())?,
            Self::BGD => writeln!(f, "{}, country_code = 50 , name = Bangladesh , alpha_2 = BD ", self.as_variant_str())?,
            Self::BRB => writeln!(f, "{}, country_code = 52 , name = Barbados , alpha_2 = BB ", self.as_variant_str())?,
            Self::BLR => writeln!(f, "{}, country_code = 112 , name = Belarus , alpha_2 = BY ", self.as_variant_str())?,
            Self::BEL => writeln!(f, "{}, country_code = 56 , name = Belgium , alpha_2 = BE ", self.as_variant_str())?,
            Self::BLZ => writeln!(f, "{}, country_code = 84 , name = Belize , alpha_2 = BZ ", self.as_variant_str())?,
            Self::BEN => writeln!(f, "{}, country_code = 204 , name = Benin , alpha_2 = BJ ", self.as_variant_str())?,
            Self::BMU => writeln!(f, "{}, country_code = 60 , name = Bermuda , alpha_2 = BM ", self.as_variant_str())?,
            Self::BTN => writeln!(f, "{}, country_code = 64 , name = Bhutan , alpha_2 = BT ", self.as_variant_str())?,
            Self::BOL => writeln!(f, "{}, country_code = 68 , name = Bolivia (Plurinational State of) , alpha_2 = BO ", self.as_variant_str())?,
            Self::BES => writeln!(f, "{}, country_code = 535 , name = Bonaire, Sint Eustatius and Saba , alpha_2 = BQ ", self.as_variant_str())?,
            Self::BIH => writeln!(f, "{}, country_code = 70 , name = Bosnia and Herzegovina , alpha_2 = BA ", self.as_variant_str())?,
            Self::BWA => writeln!(f, "{}, country_code = 72 , name = Botswana , alpha_2 = BW ", self.as_variant_str())?,
            Self::BVT => writeln!(f, "{}, country_code = 74 , name = Bouvet Island , alpha_2 = BV ", self.as_variant_str())?,
            Self::BRA => writeln!(f, "{}, country_code = 76 , name = Brazil , alpha_2 = BR ", self.as_variant_str())?,
            Self::IOT => writeln!(f, "{}, country_code = 86 , name = British Indian Ocean Territory , alpha_2 = IO ", self.as_variant_str())?,
            Self::BRN => writeln!(f, "{}, country_code = 96 , name = Brunei Darussalam , alpha_2 = BN ", self.as_variant_str())?,
            Self::BGR => writeln!(f, "{}, country_code = 100 , name = Bulgaria , alpha_2 = BG ", self.as_variant_str())?,
            Self::BFA => writeln!(f, "{}, country_code = 854 , name = Burkina Faso , alpha_2 = BF ", self.as_variant_str())?,
            Self::BDI => writeln!(f, "{}, country_code = 108 , name = Burundi , alpha_2 = BI ", self.as_variant_str())?,
            Self::CPV => writeln!(f, "{}, country_code = 132 , name = Cabo Verde , alpha_2 = CV ", self.as_variant_str())?,
            Self::KHM => writeln!(f, "{}, country_code = 116 , name = Cambodia , alpha_2 = KH ", self.as_variant_str())?,
            Self::CMR => writeln!(f, "{}, country_code = 120 , name = Cameroon , alpha_2 = CM ", self.as_variant_str())?,
            Self::CAN => writeln!(f, "{}, country_code = 124 , name = Canada , alpha_2 = CA ", self.as_variant_str())?,
            Self::CYM => writeln!(f, "{}, country_code = 136 , name = Cayman Islands , alpha_2 = KY ", self.as_variant_str())?,
            Self::CAF => writeln!(f, "{}, country_code = 140 , name = Central African Republic , alpha_2 = CF ", self.as_variant_str())?,
            Self::TCD => writeln!(f, "{}, country_code = 148 , name = Chad , alpha_2 = TD ", self.as_variant_str())?,
            Self::CHL => writeln!(f, "{}, country_code = 152 , name = Chile , alpha_2 = CL ", self.as_variant_str())?,
            Self::CHN => writeln!(f, "{}, country_code = 156 , name = China , alpha_2 = CN ", self.as_variant_str())?,
            Self::CXR => writeln!(f, "{}, country_code = 162 , name = Christmas Island , alpha_2 = CX ", self.as_variant_str())?,
            Self::CCK => writeln!(f, "{}, country_code = 166 , name = Cocos (Keeling) Islands , alpha_2 = CC ", self.as_variant_str())?,
            Self::COL => writeln!(f, "{}, country_code = 170 , name = Colombia , alpha_2 = CO ", self.as_variant_str())?,
            Self::COM => writeln!(f, "{}, country_code = 174 , name = Comoros , alpha_2 = KM ", self.as_variant_str())?,
            Self::COG => writeln!(f, "{}, country_code = 178 , name = Congo , alpha_2 = CG ", self.as_variant_str())?,
            Self::COD => writeln!(f, "{}, country_code = 180 , name = Congo, Democratic Republic of the , alpha_2 = CD ", self.as_variant_str())?,
            Self::COK => writeln!(f, "{}, country_code = 184 , name = Cook Islands , alpha_2 = CK ", self.as_variant_str())?,
            Self::CRI => writeln!(f, "{}, country_code = 188 , name = Costa Rica , alpha_2 = CR ", self.as_variant_str())?,
            Self::CIV => writeln!(f, "{}, country_code = 384 , name = CÃ´te d'Ivoire , alpha_2 = CI ", self.as_variant_str())?,
            Self::HRV => writeln!(f, "{}, country_code = 191 , name = Croatia , alpha_2 = HR ", self.as_variant_str())?,
            Self::CUB => writeln!(f, "{}, country_code = 192 , name = Cuba , alpha_2 = CU ", self.as_variant_str())?,
            Self::CUW => writeln!(f, "{}, country_code = 531 , name = CuraÃ§ao , alpha_2 = CW ", self.as_variant_str())?,
            Self::CYP => writeln!(f, "{}, country_code = 196 , name = Cyprus , alpha_2 = CY ", self.as_variant_str())?,
            Self::CZE => writeln!(f, "{}, country_code = 203 , name = Czechia , alpha_2 = CZ ", self.as_variant_str())?,
            Self::DNK => writeln!(f, "{}, country_code = 208 , name = Denmark , alpha_2 = DK ", self.as_variant_str())?,
            Self::DJI => writeln!(f, "{}, country_code = 262 , name = Djibouti , alpha_2 = DJ ", self.as_variant_str())?,
            Self::DMA => writeln!(f, "{}, country_code = 212 , name = Dominica , alpha_2 = DM ", self.as_variant_str())?,
            Self::DOM => writeln!(f, "{}, country_code = 214 , name = Dominican Republic , alpha_2 = DO ", self.as_variant_str())?,
            Self::ECU => writeln!(f, "{}, country_code = 218 , name = Ecuador , alpha_2 = EC ", self.as_variant_str())?,
            Self::EGY => writeln!(f, "{}, country_code = 818 , name = Egypt , alpha_2 = EG ", self.as_variant_str())?,
            Self::SLV => writeln!(f, "{}, country_code = 222 , name = El Salvador , alpha_2 = SV ", self.as_variant_str())?,
            Self::GNQ => writeln!(f, "{}, country_code = 226 , name = Equatorial Guinea , alpha_2 = GQ ", self.as_variant_str())?,
            Self::ERI => writeln!(f, "{}, country_code = 232 , name = Eritrea , alpha_2 = ER ", self.as_variant_str())?,
            Self::EST => writeln!(f, "{}, country_code = 233 , name = Estonia , alpha_2 = EE ", self.as_variant_str())?,
            Self::SWZ => writeln!(f, "{}, country_code = 748 , name = Eswatini , alpha_2 = SZ ", self.as_variant_str())?,
            Self::ETH => writeln!(f, "{}, country_code = 231 , name = Ethiopia , alpha_2 = ET ", self.as_variant_str())?,
            Self::FLK => writeln!(f, "{}, country_code = 238 , name = Falkland Islands (Malvinas) , alpha_2 = FK ", self.as_variant_str())?,
            Self::FRO => writeln!(f, "{}, country_code = 234 , name = Faroe Islands , alpha_2 = FO ", self.as_variant_str())?,
            Self::FJI => writeln!(f, "{}, country_code = 242 , name = Fiji , alpha_2 = FJ ", self.as_variant_str())?,
            Self::FIN => writeln!(f, "{}, country_code = 246 , name = Finland , alpha_2 = FI ", self.as_variant_str())?,
            Self::FRA => writeln!(f, "{}, country_code = 250 , name = France , alpha_2 = FR ", self.as_variant_str())?,
            Self::GUF => writeln!(f, "{}, country_code = 254 , name = French Guiana , alpha_2 = GF ", self.as_variant_str())?,
            Self::PYF => writeln!(f, "{}, country_code = 258 , name = French Polynesia , alpha_2 = PF ", self.as_variant_str())?,
            Self::ATF => writeln!(f, "{}, country_code = 260 , name = French Southern Territories , alpha_2 = TF ", self.as_variant_str())?,
            Self::GAB => writeln!(f, "{}, country_code = 266 , name = Gabon , alpha_2 = GA ", self.as_variant_str())?,
            Self::GMB => writeln!(f, "{}, country_code = 270 , name = Gambia , alpha_2 = GM ", self.as_variant_str())?,
            Self::GEO => writeln!(f, "{}, country_code = 268 , name = Georgia , alpha_2 = GE ", self.as_variant_str())?,
            Self::DEU => writeln!(f, "{}, country_code = 276 , name = Germany , alpha_2 = DE ", self.as_variant_str())?,
            Self::GHA => writeln!(f, "{}, country_code = 288 , name = Ghana , alpha_2 = GH ", self.as_variant_str())?,
            Self::GIB => writeln!(f, "{}, country_code = 292 , name = Gibraltar , alpha_2 = GI ", self.as_variant_str())?,
            Self::GRC => writeln!(f, "{}, country_code = 300 , name = Greece , alpha_2 = GR ", self.as_variant_str())?,
            Self::GRL => writeln!(f, "{}, country_code = 304 , name = Greenland , alpha_2 = GL ", self.as_variant_str())?,
            Self::GRD => writeln!(f, "{}, country_code = 308 , name = Grenada , alpha_2 = GD ", self.as_variant_str())?,
            Self::GLP => writeln!(f, "{}, country_code = 312 , name = Guadeloupe , alpha_2 = GP ", self.as_variant_str())?,
            Self::GUM => writeln!(f, "{}, country_code = 316 , name = Guam , alpha_2 = GU ", self.as_variant_str())?,
            Self::GTM => writeln!(f, "{}, country_code = 320 , name = Guatemala , alpha_2 = GT ", self.as_variant_str())?,
            Self::GGY => writeln!(f, "{}, country_code = 831 , name = Guernsey , alpha_2 = GG ", self.as_variant_str())?,
            Self::GIN => writeln!(f, "{}, country_code = 324 , name = Guinea , alpha_2 = GN ", self.as_variant_str())?,
            Self::GNB => writeln!(f, "{}, country_code = 624 , name = Guinea-Bissau , alpha_2 = GW ", self.as_variant_str())?,
            Self::GUY => writeln!(f, "{}, country_code = 328 , name = Guyana , alpha_2 = GY ", self.as_variant_str())?,
            Self::HTI => writeln!(f, "{}, country_code = 332 , name = Haiti , alpha_2 = HT ", self.as_variant_str())?,
            Self::HMD => writeln!(f, "{}, country_code = 334 , name = Heard Island and McDonald Islands , alpha_2 = HM ", self.as_variant_str())?,
            Self::VAT => writeln!(f, "{}, country_code = 336 , name = Holy See , alpha_2 = VA ", self.as_variant_str())?,
            Self::HND => writeln!(f, "{}, country_code = 340 , name = Honduras , alpha_2 = HN ", self.as_variant_str())?,
            Self::HKG => writeln!(f, "{}, country_code = 344 , name = Hong Kong , alpha_2 = HK ", self.as_variant_str())?,
            Self::HUN => writeln!(f, "{}, country_code = 348 , name = Hungary , alpha_2 = HU ", self.as_variant_str())?,
            Self::ISL => writeln!(f, "{}, country_code = 352 , name = Iceland , alpha_2 = IS ", self.as_variant_str())?,
            Self::IND => writeln!(f, "{}, country_code = 356 , name = India , alpha_2 = IN ", self.as_variant_str())?,
            Self::IDN => writeln!(f, "{}, country_code = 360 , name = Indonesia , alpha_2 = ID ", self.as_variant_str())?,
            Self::IRN => writeln!(f, "{}, country_code = 364 , name = Iran (Islamic Republic of) , alpha_2 = IR ", self.as_variant_str())?,
            Self::IRQ => writeln!(f, "{}, country_code = 368 , name = Iraq , alpha_2 = IQ ", self.as_variant_str())?,
            Self::IRL => writeln!(f, "{}, country_code = 372 , name = Ireland , alpha_2 = IE ", self.as_variant_str())?,
            Self::IMN => writeln!(f, "{}, country_code = 833 , name = Isle of Man , alpha_2 = IM ", self.as_variant_str())?,
            Self::ISR => writeln!(f, "{}, country_code = 376 , name = Israel , alpha_2 = IL ", self.as_variant_str())?,
            Self::ITA => writeln!(f, "{}, country_code = 380 , name = Italy , alpha_2 = IT ", self.as_variant_str())?,
            Self::JAM => writeln!(f, "{}, country_code = 388 , name = Jamaica , alpha_2 = JM ", self.as_variant_str())?,
            Self::JPN => writeln!(f, "{}, country_code = 392 , name = Japan , alpha_2 = JP ", self.as_variant_str())?,
            Self::JEY => writeln!(f, "{}, country_code = 832 , name = Jersey , alpha_2 = JE ", self.as_variant_str())?,
            Self::JOR => writeln!(f, "{}, country_code = 400 , name = Jordan , alpha_2 = JO ", self.as_variant_str())?,
            Self::KAZ => writeln!(f, "{}, country_code = 398 , name = Kazakhstan , alpha_2 = KZ ", self.as_variant_str())?,
            Self::KEN => writeln!(f, "{}, country_code = 404 , name = Kenya , alpha_2 = KE ", self.as_variant_str())?,
            Self::KIR => writeln!(f, "{}, country_code = 296 , name = Kiribati , alpha_2 = KI ", self.as_variant_str())?,
            Self::PRK => writeln!(f, "{}, country_code = 408 , name = Korea (Democratic People's Republic of) , alpha_2 = KP ", self.as_variant_str())?,
            Self::KOR => writeln!(f, "{}, country_code = 410 , name = Korea, Republic of , alpha_2 = KR ", self.as_variant_str())?,
            Self::KWT => writeln!(f, "{}, country_code = 414 , name = Kuwait , alpha_2 = KW ", self.as_variant_str())?,
            Self::KGZ => writeln!(f, "{}, country_code = 417 , name = Kyrgyzstan , alpha_2 = KG ", self.as_variant_str())?,
            Self::LAO => writeln!(f, "{}, country_code = 418 , name = Lao People's Democratic Republic , alpha_2 = LA ", self.as_variant_str())?,
            Self::LVA => writeln!(f, "{}, country_code = 428 , name = Latvia , alpha_2 = LV ", self.as_variant_str())?,
            Self::LBN => writeln!(f, "{}, country_code = 422 , name = Lebanon , alpha_2 = LB ", self.as_variant_str())?,
            Self::LSO => writeln!(f, "{}, country_code = 426 , name = Lesotho , alpha_2 = LS ", self.as_variant_str())?,
            Self::LBR => writeln!(f, "{}, country_code = 430 , name = Liberia , alpha_2 = LR ", self.as_variant_str())?,
            Self::LBY => writeln!(f, "{}, country_code = 434 , name = Libya , alpha_2 = LY ", self.as_variant_str())?,
            Self::LIE => writeln!(f, "{}, country_code = 438 , name = Liechtenstein , alpha_2 = LI ", self.as_variant_str())?,
            Self::LTU => writeln!(f, "{}, country_code = 440 , name = Lithuania , alpha_2 = LT ", self.as_variant_str())?,
            Self::LUX => writeln!(f, "{}, country_code = 442 , name = Luxembourg , alpha_2 = LU ", self.as_variant_str())?,
            Self::MAC => writeln!(f, "{}, country_code = 446 , name = Macao , alpha_2 = MO ", self.as_variant_str())?,
            Self::MDG => writeln!(f, "{}, country_code = 450 , name = Madagascar , alpha_2 = MG ", self.as_variant_str())?,
            Self::MWI => writeln!(f, "{}, country_code = 454 , name = Malawi , alpha_2 = MW ", self.as_variant_str())?,
            Self::MYS => writeln!(f, "{}, country_code = 458 , name = Malaysia , alpha_2 = MY ", self.as_variant_str())?,
            Self::MDV => writeln!(f, "{}, country_code = 462 , name = Maldives , alpha_2 = MV ", self.as_variant_str())?,
            Self::MLI => writeln!(f, "{}, country_code = 466 , name = Mali , alpha_2 = ML ", self.as_variant_str())?,
            Self::MLT => writeln!(f, "{}, country_code = 470 , name = Malta , alpha_2 = MT ", self.as_variant_str())?,
            Self::MHL => writeln!(f, "{}, country_code = 584 , name = Marshall Islands , alpha_2 = MH ", self.as_variant_str())?,
            Self::MTQ => writeln!(f, "{}, country_code = 474 , name = Martinique , alpha_2 = MQ ", self.as_variant_str())?,
            Self::MRT => writeln!(f, "{}, country_code = 478 , name = Mauritania , alpha_2 = MR ", self.as_variant_str())?,
            Self::MUS => writeln!(f, "{}, country_code = 480 , name = Mauritius , alpha_2 = MU ", self.as_variant_str())?,
            Self::MYT => writeln!(f, "{}, country_code = 175 , name = Mayotte , alpha_2 = YT ", self.as_variant_str())?,
            Self::MEX => writeln!(f, "{}, country_code = 484 , name = Mexico , alpha_2 = MX ", self.as_variant_str())?,
            Self::FSM => writeln!(f, "{}, country_code = 583 , name = Micronesia (Federated States of) , alpha_2 = FM ", self.as_variant_str())?,
            Self::MDA => writeln!(f, "{}, country_code = 498 , name = Moldova, Republic of , alpha_2 = MD ", self.as_variant_str())?,
            Self::MCO => writeln!(f, "{}, country_code = 492 , name = Monaco , alpha_2 = MC ", self.as_variant_str())?,
            Self::MNG => writeln!(f, "{}, country_code = 496 , name = Mongolia , alpha_2 = MN ", self.as_variant_str())?,
            Self::MNE => writeln!(f, "{}, country_code = 499 , name = Montenegro , alpha_2 = ME ", self.as_variant_str())?,
            Self::MSR => writeln!(f, "{}, country_code = 500 , name = Montserrat , alpha_2 = MS ", self.as_variant_str())?,
            Self::MAR => writeln!(f, "{}, country_code = 504 , name = Morocco , alpha_2 = MA ", self.as_variant_str())?,
            Self::MOZ => writeln!(f, "{}, country_code = 508 , name = Mozambique , alpha_2 = MZ ", self.as_variant_str())?,
            Self::MMR => writeln!(f, "{}, country_code = 104 , name = Myanmar , alpha_2 = MM ", self.as_variant_str())?,
            Self::NAM => writeln!(f, "{}, country_code = 516 , name = Namibia , alpha_2 = NA ", self.as_variant_str())?,
            Self::NRU => writeln!(f, "{}, country_code = 520 , name = Nauru , alpha_2 = NR ", self.as_variant_str())?,
            Self::NPL => writeln!(f, "{}, country_code = 524 , name = Nepal , alpha_2 = NP ", self.as_variant_str())?,
            Self::NLD => writeln!(f, "{}, country_code = 528 , name = Netherlands , alpha_2 = NL ", self.as_variant_str())?,
            Self::NCL => writeln!(f, "{}, country_code = 540 , name = New Caledonia , alpha_2 = NC ", self.as_variant_str())?,
            Self::NZL => writeln!(f, "{}, country_code = 554 , name = New Zealand , alpha_2 = NZ ", self.as_variant_str())?,
            Self::NIC => writeln!(f, "{}, country_code = 558 , name = Nicaragua , alpha_2 = NI ", self.as_variant_str())?,
            Self::NER => writeln!(f, "{}, country_code = 562 , name = Niger , alpha_2 = NE ", self.as_variant_str())?,
            Self::NGA => writeln!(f, "{}, country_code = 566 , name = Nigeria , alpha_2 = NG ", self.as_variant_str())?,
            Self::NIU => writeln!(f, "{}, country_code = 570 , name = Niue , alpha_2 = NU ", self.as_variant_str())?,
            Self::NFK => writeln!(f, "{}, country_code = 574 , name = Norfolk Island , alpha_2 = NF ", self.as_variant_str())?,
            Self::MKD => writeln!(f, "{}, country_code = 807 , name = North Macedonia , alpha_2 = MK ", self.as_variant_str())?,
            Self::MNP => writeln!(f, "{}, country_code = 580 , name = Northern Mariana Islands , alpha_2 = MP ", self.as_variant_str())?,
            Self::NOR => writeln!(f, "{}, country_code = 578 , name = Norway , alpha_2 = NO ", self.as_variant_str())?,
            Self::OMN => writeln!(f, "{}, country_code = 512 , name = Oman , alpha_2 = OM ", self.as_variant_str())?,
            Self::PAK => writeln!(f, "{}, country_code = 586 , name = Pakistan , alpha_2 = PK ", self.as_variant_str())?,
            Self::PLW => writeln!(f, "{}, country_code = 585 , name = Palau , alpha_2 = PW ", self.as_variant_str())?,
            Self::PSE => writeln!(f, "{}, country_code = 275 , name = Palestine, State of , alpha_2 = PS ", self.as_variant_str())?,
            Self::PAN => writeln!(f, "{}, country_code = 591 , name = Panama , alpha_2 = PA ", self.as_variant_str())?,
            Self::PNG => writeln!(f, "{}, country_code = 598 , name = Papua New Guinea , alpha_2 = PG ", self.as_variant_str())?,
            Self::PRY => writeln!(f, "{}, country_code = 600 , name = Paraguay , alpha_2 = PY ", self.as_variant_str())?,
            Self::PER => writeln!(f, "{}, country_code = 604 , name = Peru , alpha_2 = PE ", self.as_variant_str())?,
            Self::PHL => writeln!(f, "{}, country_code = 608 , name = Philippines , alpha_2 = PH ", self.as_variant_str())?,
            Self::PCN => writeln!(f, "{}, country_code = 612 , name = Pitcairn , alpha_2 = PN ", self.as_variant_str())?,
            Self::POL => writeln!(f, "{}, country_code = 616 , name = Poland , alpha_2 = PL ", self.as_variant_str())?,
            Self::PRT => writeln!(f, "{}, country_code = 620 , name = Portugal , alpha_2 = PT ", self.as_variant_str())?,
            Self::PRI => writeln!(f, "{}, country_code = 630 , name = Puerto Rico , alpha_2 = PR ", self.as_variant_str())?,
            Self::QAT => writeln!(f, "{}, country_code = 634 , name = Qatar , alpha_2 = QA ", self.as_variant_str())?,
            Self::REU => writeln!(f, "{}, country_code = 638 , name = RÃ©union , alpha_2 = RE ", self.as_variant_str())?,
            Self::ROU => writeln!(f, "{}, country_code = 642 , name = Romania , alpha_2 = RO ", self.as_variant_str())?,
            Self::RUS => writeln!(f, "{}, country_code = 643 , name = Russian Federation , alpha_2 = RU ", self.as_variant_str())?,
            Self::RWA => writeln!(f, "{}, country_code = 646 , name = Rwanda , alpha_2 = RW ", self.as_variant_str())?,
            Self::BLM => writeln!(f, "{}, country_code = 652 , name = Saint BarthÃ©lemy , alpha_2 = BL ", self.as_variant_str())?,
            Self::SHN => writeln!(f, "{}, country_code = 654 , name = Saint Helena, Ascension and Tristan da Cunha , alpha_2 = SH ", self.as_variant_str())?,
            Self::KNA => writeln!(f, "{}, country_code = 659 , name = Saint Kitts and Nevis , alpha_2 = KN ", self.as_variant_str())?,
            Self::LCA => writeln!(f, "{}, country_code = 662 , name = Saint Lucia , alpha_2 = LC ", self.as_variant_str())?,
            Self::MAF => writeln!(f, "{}, country_code = 663 , name = Saint Martin (French part) , alpha_2 = MF ", self.as_variant_str())?,
            Self::SPM => writeln!(f, "{}, country_code = 666 , name = Saint Pierre and Miquelon , alpha_2 = PM ", self.as_variant_str())?,
            Self::VCT => writeln!(f, "{}, country_code = 670 , name = Saint Vincent and the Grenadines , alpha_2 = VC ", self.as_variant_str())?,
            Self::WSM => writeln!(f, "{}, country_code = 882 , name = Samoa , alpha_2 = WS ", self.as_variant_str())?,
            Self::SMR => writeln!(f, "{}, country_code = 674 , name = San Marino , alpha_2 = SM ", self.as_variant_str())?,
            Self::STP => writeln!(f, "{}, country_code = 678 , name = Sao Tome and Principe , alpha_2 = ST ", self.as_variant_str())?,
            Self::SAU => writeln!(f, "{}, country_code = 682 , name = Saudi Arabia , alpha_2 = SA ", self.as_variant_str())?,
            Self::SEN => writeln!(f, "{}, country_code = 686 , name = Senegal , alpha_2 = SN ", self.as_variant_str())?,
            Self::SRB => writeln!(f, "{}, country_code = 688 , name = Serbia , alpha_2 = RS ", self.as_variant_str())?,
            Self::SYC => writeln!(f, "{}, country_code = 690 , name = Seychelles , alpha_2 = SC ", self.as_variant_str())?,
            Self::SLE => writeln!(f, "{}, country_code = 694 , name = Sierra Leone , alpha_2 = SL ", self.as_variant_str())?,
            Self::SGP => writeln!(f, "{}, country_code = 702 , name = Singapore , alpha_2 = SG ", self.as_variant_str())?,
            Self::SXM => writeln!(f, "{}, country_code = 534 , name = Sint Maarten (Dutch part) , alpha_2 = SX ", self.as_variant_str())?,
            Self::SVK => writeln!(f, "{}, country_code = 703 , name = Slovakia , alpha_2 = SK ", self.as_variant_str())?,
            Self::SVN => writeln!(f, "{}, country_code = 705 , name = Slovenia , alpha_2 = SI ", self.as_variant_str())?,
            Self::SLB => writeln!(f, "{}, country_code = 90 , name = Solomon Islands , alpha_2 = SB ", self.as_variant_str())?,
            Self::SOM => writeln!(f, "{}, country_code = 706 , name = Somalia , alpha_2 = SO ", self.as_variant_str())?,
            Self::ZAF => writeln!(f, "{}, country_code = 710 , name = South Africa , alpha_2 = ZA ", self.as_variant_str())?,
            Self::SGS => writeln!(f, "{}, country_code = 239 , name = South Georgia and the South Sandwich Islands , alpha_2 = GS ", self.as_variant_str())?,
            Self::SSD => writeln!(f, "{}, country_code = 728 , name = South Sudan , alpha_2 = SS ", self.as_variant_str())?,
            Self::ESP => writeln!(f, "{}, country_code = 724 , name = Spain , alpha_2 = ES ", self.as_variant_str())?,
            Self::LKA => writeln!(f, "{}, country_code = 144 , name = Sri Lanka , alpha_2 = LK ", self.as_variant_str())?,
            Self::SDN => writeln!(f, "{}, country_code = 729 , name = Sudan , alpha_2 = SD ", self.as_variant_str())?,
            Self::SUR => writeln!(f, "{}, country_code = 740 , name = Suriname , alpha_2 = SR ", self.as_variant_str())?,
            Self::SJM => writeln!(f, "{}, country_code = 744 , name = Svalbard and Jan Mayen , alpha_2 = SJ ", self.as_variant_str())?,
            Self::SWE => writeln!(f, "{}, country_code = 752 , name = Sweden , alpha_2 = SE ", self.as_variant_str())?,
            Self::CHE => writeln!(f, "{}, country_code = 756 , name = Switzerland , alpha_2 = CH ", self.as_variant_str())?,
            Self::SYR => writeln!(f, "{}, country_code = 760 , name = Syrian Arab Republic , alpha_2 = SY ", self.as_variant_str())?,
            Self::TWN => writeln!(f, "{}, country_code = 158 , name = Taiwan, Province of China , alpha_2 = TW ", self.as_variant_str())?,
            Self::TJK => writeln!(f, "{}, country_code = 762 , name = Tajikistan , alpha_2 = TJ ", self.as_variant_str())?,
            Self::TZA => writeln!(f, "{}, country_code = 834 , name = Tanzania, United Republic of , alpha_2 = TZ ", self.as_variant_str())?,
            Self::THA => writeln!(f, "{}, country_code = 764 , name = Thailand , alpha_2 = TH ", self.as_variant_str())?,
            Self::TLS => writeln!(f, "{}, country_code = 626 , name = Timor-Leste , alpha_2 = TL ", self.as_variant_str())?,
            Self::TGO => writeln!(f, "{}, country_code = 768 , name = Togo , alpha_2 = TG ", self.as_variant_str())?,
            Self::TKL => writeln!(f, "{}, country_code = 772 , name = Tokelau , alpha_2 = TK ", self.as_variant_str())?,
            Self::TON => writeln!(f, "{}, country_code = 776 , name = Tonga , alpha_2 = TO ", self.as_variant_str())?,
            Self::TTO => writeln!(f, "{}, country_code = 780 , name = Trinidad and Tobago , alpha_2 = TT ", self.as_variant_str())?,
            Self::TUN => writeln!(f, "{}, country_code = 788 , name = Tunisia , alpha_2 = TN ", self.as_variant_str())?,
            Self::TUR => writeln!(f, "{}, country_code = 792 , name = Turkey , alpha_2 = TR ", self.as_variant_str())?,
            Self::TKM => writeln!(f, "{}, country_code = 795 , name = Turkmenistan , alpha_2 = TM ", self.as_variant_str())?,
            Self::TCA => writeln!(f, "{}, country_code = 796 , name = Turks and Caicos Islands , alpha_2 = TC ", self.as_variant_str())?,
            Self::TUV => writeln!(f, "{}, country_code = 798 , name = Tuvalu , alpha_2 = TV ", self.as_variant_str())?,
            Self::UGA => writeln!(f, "{}, country_code = 800 , name = Uganda , alpha_2 = UG ", self.as_variant_str())?,
            Self::UKR => writeln!(f, "{}, country_code = 804 , name = Ukraine , alpha_2 = UA ", self.as_variant_str())?,
            Self::ARE => writeln!(f, "{}, country_code = 784 , name = United Arab Emirates , alpha_2 = AE ", self.as_variant_str())?,
            Self::GBR => writeln!(f, "{}, country_code = 826 , name = United Kingdom of Great Britain and Northern Ireland , alpha_2 = GB ", self.as_variant_str())?,
            Self::USA => writeln!(f, "{}, country_code = 840 , name = United States of America , alpha_2 = US ", self.as_variant_str())?,
            Self::UMI => writeln!(f, "{}, country_code = 581 , name = United States Minor Outlying Islands , alpha_2 = UM ", self.as_variant_str())?,
            Self::URY => writeln!(f, "{}, country_code = 858 , name = Uruguay , alpha_2 = UY ", self.as_variant_str())?,
            Self::UZB => writeln!(f, "{}, country_code = 860 , name = Uzbekistan , alpha_2 = UZ ", self.as_variant_str())?,
            Self::VUT => writeln!(f, "{}, country_code = 548 , name = Vanuatu , alpha_2 = VU ", self.as_variant_str())?,
            Self::VEN => writeln!(f, "{}, country_code = 862 , name = Venezuela (Bolivarian Republic of) , alpha_2 = VE ", self.as_variant_str())?,
            Self::VNM => writeln!(f, "{}, country_code = 704 , name = Viet Nam , alpha_2 = VN ", self.as_variant_str())?,
            Self::VGB => writeln!(f, "{}, country_code = 92 , name = Virgin Islands (British) , alpha_2 = VG ", self.as_variant_str())?,
            Self::VIR => writeln!(f, "{}, country_code = 850 , name = Virgin Islands (U.S.) , alpha_2 = VI ", self.as_variant_str())?,
            Self::WLF => writeln!(f, "{}, country_code = 876 , name = Wallis and Futuna , alpha_2 = WF ", self.as_variant_str())?,
            Self::ESH => writeln!(f, "{}, country_code = 732 , name = Western Sahara , alpha_2 = EH ", self.as_variant_str())?,
            Self::YEM => writeln!(f, "{}, country_code = 887 , name = Yemen , alpha_2 = YE ", self.as_variant_str())?,
            Self::ZMB => writeln!(f, "{}, country_code = 894 , name = Zambia , alpha_2 = ZM ", self.as_variant_str())?,
            Self::ZWE => writeln!(f, "{}, country_code = 716 , name = Zimbabwe , alpha_2 = ZW ", self.as_variant_str())?,
        }
        Ok(())
    }
}
const COUNTRY_ALL_VARIANTS_ARRAY: [Country; 249] = [ 
Country::AFG,
Country::ALA,
Country::ALB,
Country::DZA,
Country::ASM,
Country::AND,
Country::AGO,
Country::AIA,
Country::ATA,
Country::ATG,
Country::ARG,
Country::ARM,
Country::ABW,
Country::AUS,
Country::AUT,
Country::AZE,
Country::BHS,
Country::BHR,
Country::BGD,
Country::BRB,
Country::BLR,
Country::BEL,
Country::BLZ,
Country::BEN,
Country::BMU,
Country::BTN,
Country::BOL,
Country::BES,
Country::BIH,
Country::BWA,
Country::BVT,
Country::BRA,
Country::IOT,
Country::BRN,
Country::BGR,
Country::BFA,
Country::BDI,
Country::CPV,
Country::KHM,
Country::CMR,
Country::CAN,
Country::CYM,
Country::CAF,
Country::TCD,
Country::CHL,
Country::CHN,
Country::CXR,
Country::CCK,
Country::COL,
Country::COM,
Country::COG,
Country::COD,
Country::COK,
Country::CRI,
Country::CIV,
Country::HRV,
Country::CUB,
Country::CUW,
Country::CYP,
Country::CZE,
Country::DNK,
Country::DJI,
Country::DMA,
Country::DOM,
Country::ECU,
Country::EGY,
Country::SLV,
Country::GNQ,
Country::ERI,
Country::EST,
Country::SWZ,
Country::ETH,
Country::FLK,
Country::FRO,
Country::FJI,
Country::FIN,
Country::FRA,
Country::GUF,
Country::PYF,
Country::ATF,
Country::GAB,
Country::GMB,
Country::GEO,
Country::DEU,
Country::GHA,
Country::GIB,
Country::GRC,
Country::GRL,
Country::GRD,
Country::GLP,
Country::GUM,
Country::GTM,
Country::GGY,
Country::GIN,
Country::GNB,
Country::GUY,
Country::HTI,
Country::HMD,
Country::VAT,
Country::HND,
Country::HKG,
Country::HUN,
Country::ISL,
Country::IND,
Country::IDN,
Country::IRN,
Country::IRQ,
Country::IRL,
Country::IMN,
Country::ISR,
Country::ITA,
Country::JAM,
Country::JPN,
Country::JEY,
Country::JOR,
Country::KAZ,
Country::KEN,
Country::KIR,
Country::PRK,
Country::KOR,
Country::KWT,
Country::KGZ,
Country::LAO,
Country::LVA,
Country::LBN,
Country::LSO,
Country::LBR,
Country::LBY,
Country::LIE,
Country::LTU,
Country::LUX,
Country::MAC,
Country::MDG,
Country::MWI,
Country::MYS,
Country::MDV,
Country::MLI,
Country::MLT,
Country::MHL,
Country::MTQ,
Country::MRT,
Country::MUS,
Country::MYT,
Country::MEX,
Country::FSM,
Country::MDA,
Country::MCO,
Country::MNG,
Country::MNE,
Country::MSR,
Country::MAR,
Country::MOZ,
Country::MMR,
Country::NAM,
Country::NRU,
Country::NPL,
Country::NLD,
Country::NCL,
Country::NZL,
Country::NIC,
Country::NER,
Country::NGA,
Country::NIU,
Country::NFK,
Country::MKD,
Country::MNP,
Country::NOR,
Country::OMN,
Country::PAK,
Country::PLW,
Country::PSE,
Country::PAN,
Country::PNG,
Country::PRY,
Country::PER,
Country::PHL,
Country::PCN,
Country::POL,
Country::PRT,
Country::PRI,
Country::QAT,
Country::REU,
Country::ROU,
Country::RUS,
Country::RWA,
Country::BLM,
Country::SHN,
Country::KNA,
Country::LCA,
Country::MAF,
Country::SPM,
Country::VCT,
Country::WSM,
Country::SMR,
Country::STP,
Country::SAU,
Country::SEN,
Country::SRB,
Country::SYC,
Country::SLE,
Country::SGP,
Country::SXM,
Country::SVK,
Country::SVN,
Country::SLB,
Country::SOM,
Country::ZAF,
Country::SGS,
Country::SSD,
Country::ESP,
Country::LKA,
Country::SDN,
Country::SUR,
Country::SJM,
Country::SWE,
Country::CHE,
Country::SYR,
Country::TWN,
Country::TJK,
Country::TZA,
Country::THA,
Country::TLS,
Country::TGO,
Country::TKL,
Country::TON,
Country::TTO,
Country::TUN,
Country::TUR,
Country::TKM,
Country::TCA,
Country::TUV,
Country::UGA,
Country::UKR,
Country::ARE,
Country::GBR,
Country::USA,
Country::UMI,
Country::URY,
Country::UZB,
Country::VUT,
Country::VEN,
Country::VNM,
Country::VGB,
Country::VIR,
Country::WLF,
Country::ESH,
Country::YEM,
Country::ZMB,
Country::ZWE,
];
pub const fn country_get_all_variants() -> [Country; 249]
{
    COUNTRY_ALL_VARIANTS_ARRAY
}
// Variant string representation.
const COUNTRY_AFG_STR:&'static str = "AFG";
const COUNTRY_ALA_STR:&'static str = "ALA";
const COUNTRY_ALB_STR:&'static str = "ALB";
const COUNTRY_DZA_STR:&'static str = "DZA";
const COUNTRY_ASM_STR:&'static str = "ASM";
const COUNTRY_AND_STR:&'static str = "AND";
const COUNTRY_AGO_STR:&'static str = "AGO";
const COUNTRY_AIA_STR:&'static str = "AIA";
const COUNTRY_ATA_STR:&'static str = "ATA";
const COUNTRY_ATG_STR:&'static str = "ATG";
const COUNTRY_ARG_STR:&'static str = "ARG";
const COUNTRY_ARM_STR:&'static str = "ARM";
const COUNTRY_ABW_STR:&'static str = "ABW";
const COUNTRY_AUS_STR:&'static str = "AUS";
const COUNTRY_AUT_STR:&'static str = "AUT";
const COUNTRY_AZE_STR:&'static str = "AZE";
const COUNTRY_BHS_STR:&'static str = "BHS";
const COUNTRY_BHR_STR:&'static str = "BHR";
const COUNTRY_BGD_STR:&'static str = "BGD";
const COUNTRY_BRB_STR:&'static str = "BRB";
const COUNTRY_BLR_STR:&'static str = "BLR";
const COUNTRY_BEL_STR:&'static str = "BEL";
const COUNTRY_BLZ_STR:&'static str = "BLZ";
const COUNTRY_BEN_STR:&'static str = "BEN";
const COUNTRY_BMU_STR:&'static str = "BMU";
const COUNTRY_BTN_STR:&'static str = "BTN";
const COUNTRY_BOL_STR:&'static str = "BOL";
const COUNTRY_BES_STR:&'static str = "BES";
const COUNTRY_BIH_STR:&'static str = "BIH";
const COUNTRY_BWA_STR:&'static str = "BWA";
const COUNTRY_BVT_STR:&'static str = "BVT";
const COUNTRY_BRA_STR:&'static str = "BRA";
const COUNTRY_IOT_STR:&'static str = "IOT";
const COUNTRY_BRN_STR:&'static str = "BRN";
const COUNTRY_BGR_STR:&'static str = "BGR";
const COUNTRY_BFA_STR:&'static str = "BFA";
const COUNTRY_BDI_STR:&'static str = "BDI";
const COUNTRY_CPV_STR:&'static str = "CPV";
const COUNTRY_KHM_STR:&'static str = "KHM";
const COUNTRY_CMR_STR:&'static str = "CMR";
const COUNTRY_CAN_STR:&'static str = "CAN";
const COUNTRY_CYM_STR:&'static str = "CYM";
const COUNTRY_CAF_STR:&'static str = "CAF";
const COUNTRY_TCD_STR:&'static str = "TCD";
const COUNTRY_CHL_STR:&'static str = "CHL";
const COUNTRY_CHN_STR:&'static str = "CHN";
const COUNTRY_CXR_STR:&'static str = "CXR";
const COUNTRY_CCK_STR:&'static str = "CCK";
const COUNTRY_COL_STR:&'static str = "COL";
const COUNTRY_COM_STR:&'static str = "COM";
const COUNTRY_COG_STR:&'static str = "COG";
const COUNTRY_COD_STR:&'static str = "COD";
const COUNTRY_COK_STR:&'static str = "COK";
const COUNTRY_CRI_STR:&'static str = "CRI";
const COUNTRY_CIV_STR:&'static str = "CIV";
const COUNTRY_HRV_STR:&'static str = "HRV";
const COUNTRY_CUB_STR:&'static str = "CUB";
const COUNTRY_CUW_STR:&'static str = "CUW";
const COUNTRY_CYP_STR:&'static str = "CYP";
const COUNTRY_CZE_STR:&'static str = "CZE";
const COUNTRY_DNK_STR:&'static str = "DNK";
const COUNTRY_DJI_STR:&'static str = "DJI";
const COUNTRY_DMA_STR:&'static str = "DMA";
const COUNTRY_DOM_STR:&'static str = "DOM";
const COUNTRY_ECU_STR:&'static str = "ECU";
const COUNTRY_EGY_STR:&'static str = "EGY";
const COUNTRY_SLV_STR:&'static str = "SLV";
const COUNTRY_GNQ_STR:&'static str = "GNQ";
const COUNTRY_ERI_STR:&'static str = "ERI";
const COUNTRY_EST_STR:&'static str = "EST";
const COUNTRY_SWZ_STR:&'static str = "SWZ";
const COUNTRY_ETH_STR:&'static str = "ETH";
const COUNTRY_FLK_STR:&'static str = "FLK";
const COUNTRY_FRO_STR:&'static str = "FRO";
const COUNTRY_FJI_STR:&'static str = "FJI";
const COUNTRY_FIN_STR:&'static str = "FIN";
const COUNTRY_FRA_STR:&'static str = "FRA";
const COUNTRY_GUF_STR:&'static str = "GUF";
const COUNTRY_PYF_STR:&'static str = "PYF";
const COUNTRY_ATF_STR:&'static str = "ATF";
const COUNTRY_GAB_STR:&'static str = "GAB";
const COUNTRY_GMB_STR:&'static str = "GMB";
const COUNTRY_GEO_STR:&'static str = "GEO";
const COUNTRY_DEU_STR:&'static str = "DEU";
const COUNTRY_GHA_STR:&'static str = "GHA";
const COUNTRY_GIB_STR:&'static str = "GIB";
const COUNTRY_GRC_STR:&'static str = "GRC";
const COUNTRY_GRL_STR:&'static str = "GRL";
const COUNTRY_GRD_STR:&'static str = "GRD";
const COUNTRY_GLP_STR:&'static str = "GLP";
const COUNTRY_GUM_STR:&'static str = "GUM";
const COUNTRY_GTM_STR:&'static str = "GTM";
const COUNTRY_GGY_STR:&'static str = "GGY";
const COUNTRY_GIN_STR:&'static str = "GIN";
const COUNTRY_GNB_STR:&'static str = "GNB";
const COUNTRY_GUY_STR:&'static str = "GUY";
const COUNTRY_HTI_STR:&'static str = "HTI";
const COUNTRY_HMD_STR:&'static str = "HMD";
const COUNTRY_VAT_STR:&'static str = "VAT";
const COUNTRY_HND_STR:&'static str = "HND";
const COUNTRY_HKG_STR:&'static str = "HKG";
const COUNTRY_HUN_STR:&'static str = "HUN";
const COUNTRY_ISL_STR:&'static str = "ISL";
const COUNTRY_IND_STR:&'static str = "IND";
const COUNTRY_IDN_STR:&'static str = "IDN";
const COUNTRY_IRN_STR:&'static str = "IRN";
const COUNTRY_IRQ_STR:&'static str = "IRQ";
const COUNTRY_IRL_STR:&'static str = "IRL";
const COUNTRY_IMN_STR:&'static str = "IMN";
const COUNTRY_ISR_STR:&'static str = "ISR";
const COUNTRY_ITA_STR:&'static str = "ITA";
const COUNTRY_JAM_STR:&'static str = "JAM";
const COUNTRY_JPN_STR:&'static str = "JPN";
const COUNTRY_JEY_STR:&'static str = "JEY";
const COUNTRY_JOR_STR:&'static str = "JOR";
const COUNTRY_KAZ_STR:&'static str = "KAZ";
const COUNTRY_KEN_STR:&'static str = "KEN";
const COUNTRY_KIR_STR:&'static str = "KIR";
const COUNTRY_PRK_STR:&'static str = "PRK";
const COUNTRY_KOR_STR:&'static str = "KOR";
const COUNTRY_KWT_STR:&'static str = "KWT";
const COUNTRY_KGZ_STR:&'static str = "KGZ";
const COUNTRY_LAO_STR:&'static str = "LAO";
const COUNTRY_LVA_STR:&'static str = "LVA";
const COUNTRY_LBN_STR:&'static str = "LBN";
const COUNTRY_LSO_STR:&'static str = "LSO";
const COUNTRY_LBR_STR:&'static str = "LBR";
const COUNTRY_LBY_STR:&'static str = "LBY";
const COUNTRY_LIE_STR:&'static str = "LIE";
const COUNTRY_LTU_STR:&'static str = "LTU";
const COUNTRY_LUX_STR:&'static str = "LUX";
const COUNTRY_MAC_STR:&'static str = "MAC";
const COUNTRY_MDG_STR:&'static str = "MDG";
const COUNTRY_MWI_STR:&'static str = "MWI";
const COUNTRY_MYS_STR:&'static str = "MYS";
const COUNTRY_MDV_STR:&'static str = "MDV";
const COUNTRY_MLI_STR:&'static str = "MLI";
const COUNTRY_MLT_STR:&'static str = "MLT";
const COUNTRY_MHL_STR:&'static str = "MHL";
const COUNTRY_MTQ_STR:&'static str = "MTQ";
const COUNTRY_MRT_STR:&'static str = "MRT";
const COUNTRY_MUS_STR:&'static str = "MUS";
const COUNTRY_MYT_STR:&'static str = "MYT";
const COUNTRY_MEX_STR:&'static str = "MEX";
const COUNTRY_FSM_STR:&'static str = "FSM";
const COUNTRY_MDA_STR:&'static str = "MDA";
const COUNTRY_MCO_STR:&'static str = "MCO";
const COUNTRY_MNG_STR:&'static str = "MNG";
const COUNTRY_MNE_STR:&'static str = "MNE";
const COUNTRY_MSR_STR:&'static str = "MSR";
const COUNTRY_MAR_STR:&'static str = "MAR";
const COUNTRY_MOZ_STR:&'static str = "MOZ";
const COUNTRY_MMR_STR:&'static str = "MMR";
const COUNTRY_NAM_STR:&'static str = "NAM";
const COUNTRY_NRU_STR:&'static str = "NRU";
const COUNTRY_NPL_STR:&'static str = "NPL";
const COUNTRY_NLD_STR:&'static str = "NLD";
const COUNTRY_NCL_STR:&'static str = "NCL";
const COUNTRY_NZL_STR:&'static str = "NZL";
const COUNTRY_NIC_STR:&'static str = "NIC";
const COUNTRY_NER_STR:&'static str = "NER";
const COUNTRY_NGA_STR:&'static str = "NGA";
const COUNTRY_NIU_STR:&'static str = "NIU";
const COUNTRY_NFK_STR:&'static str = "NFK";
const COUNTRY_MKD_STR:&'static str = "MKD";
const COUNTRY_MNP_STR:&'static str = "MNP";
const COUNTRY_NOR_STR:&'static str = "NOR";
const COUNTRY_OMN_STR:&'static str = "OMN";
const COUNTRY_PAK_STR:&'static str = "PAK";
const COUNTRY_PLW_STR:&'static str = "PLW";
const COUNTRY_PSE_STR:&'static str = "PSE";
const COUNTRY_PAN_STR:&'static str = "PAN";
const COUNTRY_PNG_STR:&'static str = "PNG";
const COUNTRY_PRY_STR:&'static str = "PRY";
const COUNTRY_PER_STR:&'static str = "PER";
const COUNTRY_PHL_STR:&'static str = "PHL";
const COUNTRY_PCN_STR:&'static str = "PCN";
const COUNTRY_POL_STR:&'static str = "POL";
const COUNTRY_PRT_STR:&'static str = "PRT";
const COUNTRY_PRI_STR:&'static str = "PRI";
const COUNTRY_QAT_STR:&'static str = "QAT";
const COUNTRY_REU_STR:&'static str = "REU";
const COUNTRY_ROU_STR:&'static str = "ROU";
const COUNTRY_RUS_STR:&'static str = "RUS";
const COUNTRY_RWA_STR:&'static str = "RWA";
const COUNTRY_BLM_STR:&'static str = "BLM";
const COUNTRY_SHN_STR:&'static str = "SHN";
const COUNTRY_KNA_STR:&'static str = "KNA";
const COUNTRY_LCA_STR:&'static str = "LCA";
const COUNTRY_MAF_STR:&'static str = "MAF";
const COUNTRY_SPM_STR:&'static str = "SPM";
const COUNTRY_VCT_STR:&'static str = "VCT";
const COUNTRY_WSM_STR:&'static str = "WSM";
const COUNTRY_SMR_STR:&'static str = "SMR";
const COUNTRY_STP_STR:&'static str = "STP";
const COUNTRY_SAU_STR:&'static str = "SAU";
const COUNTRY_SEN_STR:&'static str = "SEN";
const COUNTRY_SRB_STR:&'static str = "SRB";
const COUNTRY_SYC_STR:&'static str = "SYC";
const COUNTRY_SLE_STR:&'static str = "SLE";
const COUNTRY_SGP_STR:&'static str = "SGP";
const COUNTRY_SXM_STR:&'static str = "SXM";
const COUNTRY_SVK_STR:&'static str = "SVK";
const COUNTRY_SVN_STR:&'static str = "SVN";
const COUNTRY_SLB_STR:&'static str = "SLB";
const COUNTRY_SOM_STR:&'static str = "SOM";
const COUNTRY_ZAF_STR:&'static str = "ZAF";
const COUNTRY_SGS_STR:&'static str = "SGS";
const COUNTRY_SSD_STR:&'static str = "SSD";
const COUNTRY_ESP_STR:&'static str = "ESP";
const COUNTRY_LKA_STR:&'static str = "LKA";
const COUNTRY_SDN_STR:&'static str = "SDN";
const COUNTRY_SUR_STR:&'static str = "SUR";
const COUNTRY_SJM_STR:&'static str = "SJM";
const COUNTRY_SWE_STR:&'static str = "SWE";
const COUNTRY_CHE_STR:&'static str = "CHE";
const COUNTRY_SYR_STR:&'static str = "SYR";
const COUNTRY_TWN_STR:&'static str = "TWN";
const COUNTRY_TJK_STR:&'static str = "TJK";
const COUNTRY_TZA_STR:&'static str = "TZA";
const COUNTRY_THA_STR:&'static str = "THA";
const COUNTRY_TLS_STR:&'static str = "TLS";
const COUNTRY_TGO_STR:&'static str = "TGO";
const COUNTRY_TKL_STR:&'static str = "TKL";
const COUNTRY_TON_STR:&'static str = "TON";
const COUNTRY_TTO_STR:&'static str = "TTO";
const COUNTRY_TUN_STR:&'static str = "TUN";
const COUNTRY_TUR_STR:&'static str = "TUR";
const COUNTRY_TKM_STR:&'static str = "TKM";
const COUNTRY_TCA_STR:&'static str = "TCA";
const COUNTRY_TUV_STR:&'static str = "TUV";
const COUNTRY_UGA_STR:&'static str = "UGA";
const COUNTRY_UKR_STR:&'static str = "UKR";
const COUNTRY_ARE_STR:&'static str = "ARE";
const COUNTRY_GBR_STR:&'static str = "GBR";
const COUNTRY_USA_STR:&'static str = "USA";
const COUNTRY_UMI_STR:&'static str = "UMI";
const COUNTRY_URY_STR:&'static str = "URY";
const COUNTRY_UZB_STR:&'static str = "UZB";
const COUNTRY_VUT_STR:&'static str = "VUT";
const COUNTRY_VEN_STR:&'static str = "VEN";
const COUNTRY_VNM_STR:&'static str = "VNM";
const COUNTRY_VGB_STR:&'static str = "VGB";
const COUNTRY_VIR_STR:&'static str = "VIR";
const COUNTRY_WLF_STR:&'static str = "WLF";
const COUNTRY_ESH_STR:&'static str = "ESH";
const COUNTRY_YEM_STR:&'static str = "YEM";
const COUNTRY_ZMB_STR:&'static str = "ZMB";
const COUNTRY_ZWE_STR:&'static str = "ZWE";

/// Returns the variants name as a &str.
pub const fn country_as_variant_str(country: &Country) -> &'static str
{
match country {
    Country::AFG => COUNTRY_AFG_STR,
    Country::ALA => COUNTRY_ALA_STR,
    Country::ALB => COUNTRY_ALB_STR,
    Country::DZA => COUNTRY_DZA_STR,
    Country::ASM => COUNTRY_ASM_STR,
    Country::AND => COUNTRY_AND_STR,
    Country::AGO => COUNTRY_AGO_STR,
    Country::AIA => COUNTRY_AIA_STR,
    Country::ATA => COUNTRY_ATA_STR,
    Country::ATG => COUNTRY_ATG_STR,
    Country::ARG => COUNTRY_ARG_STR,
    Country::ARM => COUNTRY_ARM_STR,
    Country::ABW => COUNTRY_ABW_STR,
    Country::AUS => COUNTRY_AUS_STR,
    Country::AUT => COUNTRY_AUT_STR,
    Country::AZE => COUNTRY_AZE_STR,
    Country::BHS => COUNTRY_BHS_STR,
    Country::BHR => COUNTRY_BHR_STR,
    Country::BGD => COUNTRY_BGD_STR,
    Country::BRB => COUNTRY_BRB_STR,
    Country::BLR => COUNTRY_BLR_STR,
    Country::BEL => COUNTRY_BEL_STR,
    Country::BLZ => COUNTRY_BLZ_STR,
    Country::BEN => COUNTRY_BEN_STR,
    Country::BMU => COUNTRY_BMU_STR,
    Country::BTN => COUNTRY_BTN_STR,
    Country::BOL => COUNTRY_BOL_STR,
    Country::BES => COUNTRY_BES_STR,
    Country::BIH => COUNTRY_BIH_STR,
    Country::BWA => COUNTRY_BWA_STR,
    Country::BVT => COUNTRY_BVT_STR,
    Country::BRA => COUNTRY_BRA_STR,
    Country::IOT => COUNTRY_IOT_STR,
    Country::BRN => COUNTRY_BRN_STR,
    Country::BGR => COUNTRY_BGR_STR,
    Country::BFA => COUNTRY_BFA_STR,
    Country::BDI => COUNTRY_BDI_STR,
    Country::CPV => COUNTRY_CPV_STR,
    Country::KHM => COUNTRY_KHM_STR,
    Country::CMR => COUNTRY_CMR_STR,
    Country::CAN => COUNTRY_CAN_STR,
    Country::CYM => COUNTRY_CYM_STR,
    Country::CAF => COUNTRY_CAF_STR,
    Country::TCD => COUNTRY_TCD_STR,
    Country::CHL => COUNTRY_CHL_STR,
    Country::CHN => COUNTRY_CHN_STR,
    Country::CXR => COUNTRY_CXR_STR,
    Country::CCK => COUNTRY_CCK_STR,
    Country::COL => COUNTRY_COL_STR,
    Country::COM => COUNTRY_COM_STR,
    Country::COG => COUNTRY_COG_STR,
    Country::COD => COUNTRY_COD_STR,
    Country::COK => COUNTRY_COK_STR,
    Country::CRI => COUNTRY_CRI_STR,
    Country::CIV => COUNTRY_CIV_STR,
    Country::HRV => COUNTRY_HRV_STR,
    Country::CUB => COUNTRY_CUB_STR,
    Country::CUW => COUNTRY_CUW_STR,
    Country::CYP => COUNTRY_CYP_STR,
    Country::CZE => COUNTRY_CZE_STR,
    Country::DNK => COUNTRY_DNK_STR,
    Country::DJI => COUNTRY_DJI_STR,
    Country::DMA => COUNTRY_DMA_STR,
    Country::DOM => COUNTRY_DOM_STR,
    Country::ECU => COUNTRY_ECU_STR,
    Country::EGY => COUNTRY_EGY_STR,
    Country::SLV => COUNTRY_SLV_STR,
    Country::GNQ => COUNTRY_GNQ_STR,
    Country::ERI => COUNTRY_ERI_STR,
    Country::EST => COUNTRY_EST_STR,
    Country::SWZ => COUNTRY_SWZ_STR,
    Country::ETH => COUNTRY_ETH_STR,
    Country::FLK => COUNTRY_FLK_STR,
    Country::FRO => COUNTRY_FRO_STR,
    Country::FJI => COUNTRY_FJI_STR,
    Country::FIN => COUNTRY_FIN_STR,
    Country::FRA => COUNTRY_FRA_STR,
    Country::GUF => COUNTRY_GUF_STR,
    Country::PYF => COUNTRY_PYF_STR,
    Country::ATF => COUNTRY_ATF_STR,
    Country::GAB => COUNTRY_GAB_STR,
    Country::GMB => COUNTRY_GMB_STR,
    Country::GEO => COUNTRY_GEO_STR,
    Country::DEU => COUNTRY_DEU_STR,
    Country::GHA => COUNTRY_GHA_STR,
    Country::GIB => COUNTRY_GIB_STR,
    Country::GRC => COUNTRY_GRC_STR,
    Country::GRL => COUNTRY_GRL_STR,
    Country::GRD => COUNTRY_GRD_STR,
    Country::GLP => COUNTRY_GLP_STR,
    Country::GUM => COUNTRY_GUM_STR,
    Country::GTM => COUNTRY_GTM_STR,
    Country::GGY => COUNTRY_GGY_STR,
    Country::GIN => COUNTRY_GIN_STR,
    Country::GNB => COUNTRY_GNB_STR,
    Country::GUY => COUNTRY_GUY_STR,
    Country::HTI => COUNTRY_HTI_STR,
    Country::HMD => COUNTRY_HMD_STR,
    Country::VAT => COUNTRY_VAT_STR,
    Country::HND => COUNTRY_HND_STR,
    Country::HKG => COUNTRY_HKG_STR,
    Country::HUN => COUNTRY_HUN_STR,
    Country::ISL => COUNTRY_ISL_STR,
    Country::IND => COUNTRY_IND_STR,
    Country::IDN => COUNTRY_IDN_STR,
    Country::IRN => COUNTRY_IRN_STR,
    Country::IRQ => COUNTRY_IRQ_STR,
    Country::IRL => COUNTRY_IRL_STR,
    Country::IMN => COUNTRY_IMN_STR,
    Country::ISR => COUNTRY_ISR_STR,
    Country::ITA => COUNTRY_ITA_STR,
    Country::JAM => COUNTRY_JAM_STR,
    Country::JPN => COUNTRY_JPN_STR,
    Country::JEY => COUNTRY_JEY_STR,
    Country::JOR => COUNTRY_JOR_STR,
    Country::KAZ => COUNTRY_KAZ_STR,
    Country::KEN => COUNTRY_KEN_STR,
    Country::KIR => COUNTRY_KIR_STR,
    Country::PRK => COUNTRY_PRK_STR,
    Country::KOR => COUNTRY_KOR_STR,
    Country::KWT => COUNTRY_KWT_STR,
    Country::KGZ => COUNTRY_KGZ_STR,
    Country::LAO => COUNTRY_LAO_STR,
    Country::LVA => COUNTRY_LVA_STR,
    Country::LBN => COUNTRY_LBN_STR,
    Country::LSO => COUNTRY_LSO_STR,
    Country::LBR => COUNTRY_LBR_STR,
    Country::LBY => COUNTRY_LBY_STR,
    Country::LIE => COUNTRY_LIE_STR,
    Country::LTU => COUNTRY_LTU_STR,
    Country::LUX => COUNTRY_LUX_STR,
    Country::MAC => COUNTRY_MAC_STR,
    Country::MDG => COUNTRY_MDG_STR,
    Country::MWI => COUNTRY_MWI_STR,
    Country::MYS => COUNTRY_MYS_STR,
    Country::MDV => COUNTRY_MDV_STR,
    Country::MLI => COUNTRY_MLI_STR,
    Country::MLT => COUNTRY_MLT_STR,
    Country::MHL => COUNTRY_MHL_STR,
    Country::MTQ => COUNTRY_MTQ_STR,
    Country::MRT => COUNTRY_MRT_STR,
    Country::MUS => COUNTRY_MUS_STR,
    Country::MYT => COUNTRY_MYT_STR,
    Country::MEX => COUNTRY_MEX_STR,
    Country::FSM => COUNTRY_FSM_STR,
    Country::MDA => COUNTRY_MDA_STR,
    Country::MCO => COUNTRY_MCO_STR,
    Country::MNG => COUNTRY_MNG_STR,
    Country::MNE => COUNTRY_MNE_STR,
    Country::MSR => COUNTRY_MSR_STR,
    Country::MAR => COUNTRY_MAR_STR,
    Country::MOZ => COUNTRY_MOZ_STR,
    Country::MMR => COUNTRY_MMR_STR,
    Country::NAM => COUNTRY_NAM_STR,
    Country::NRU => COUNTRY_NRU_STR,
    Country::NPL => COUNTRY_NPL_STR,
    Country::NLD => COUNTRY_NLD_STR,
    Country::NCL => COUNTRY_NCL_STR,
    Country::NZL => COUNTRY_NZL_STR,
    Country::NIC => COUNTRY_NIC_STR,
    Country::NER => COUNTRY_NER_STR,
    Country::NGA => COUNTRY_NGA_STR,
    Country::NIU => COUNTRY_NIU_STR,
    Country::NFK => COUNTRY_NFK_STR,
    Country::MKD => COUNTRY_MKD_STR,
    Country::MNP => COUNTRY_MNP_STR,
    Country::NOR => COUNTRY_NOR_STR,
    Country::OMN => COUNTRY_OMN_STR,
    Country::PAK => COUNTRY_PAK_STR,
    Country::PLW => COUNTRY_PLW_STR,
    Country::PSE => COUNTRY_PSE_STR,
    Country::PAN => COUNTRY_PAN_STR,
    Country::PNG => COUNTRY_PNG_STR,
    Country::PRY => COUNTRY_PRY_STR,
    Country::PER => COUNTRY_PER_STR,
    Country::PHL => COUNTRY_PHL_STR,
    Country::PCN => COUNTRY_PCN_STR,
    Country::POL => COUNTRY_POL_STR,
    Country::PRT => COUNTRY_PRT_STR,
    Country::PRI => COUNTRY_PRI_STR,
    Country::QAT => COUNTRY_QAT_STR,
    Country::REU => COUNTRY_REU_STR,
    Country::ROU => COUNTRY_ROU_STR,
    Country::RUS => COUNTRY_RUS_STR,
    Country::RWA => COUNTRY_RWA_STR,
    Country::BLM => COUNTRY_BLM_STR,
    Country::SHN => COUNTRY_SHN_STR,
    Country::KNA => COUNTRY_KNA_STR,
    Country::LCA => COUNTRY_LCA_STR,
    Country::MAF => COUNTRY_MAF_STR,
    Country::SPM => COUNTRY_SPM_STR,
    Country::VCT => COUNTRY_VCT_STR,
    Country::WSM => COUNTRY_WSM_STR,
    Country::SMR => COUNTRY_SMR_STR,
    Country::STP => COUNTRY_STP_STR,
    Country::SAU => COUNTRY_SAU_STR,
    Country::SEN => COUNTRY_SEN_STR,
    Country::SRB => COUNTRY_SRB_STR,
    Country::SYC => COUNTRY_SYC_STR,
    Country::SLE => COUNTRY_SLE_STR,
    Country::SGP => COUNTRY_SGP_STR,
    Country::SXM => COUNTRY_SXM_STR,
    Country::SVK => COUNTRY_SVK_STR,
    Country::SVN => COUNTRY_SVN_STR,
    Country::SLB => COUNTRY_SLB_STR,
    Country::SOM => COUNTRY_SOM_STR,
    Country::ZAF => COUNTRY_ZAF_STR,
    Country::SGS => COUNTRY_SGS_STR,
    Country::SSD => COUNTRY_SSD_STR,
    Country::ESP => COUNTRY_ESP_STR,
    Country::LKA => COUNTRY_LKA_STR,
    Country::SDN => COUNTRY_SDN_STR,
    Country::SUR => COUNTRY_SUR_STR,
    Country::SJM => COUNTRY_SJM_STR,
    Country::SWE => COUNTRY_SWE_STR,
    Country::CHE => COUNTRY_CHE_STR,
    Country::SYR => COUNTRY_SYR_STR,
    Country::TWN => COUNTRY_TWN_STR,
    Country::TJK => COUNTRY_TJK_STR,
    Country::TZA => COUNTRY_TZA_STR,
    Country::THA => COUNTRY_THA_STR,
    Country::TLS => COUNTRY_TLS_STR,
    Country::TGO => COUNTRY_TGO_STR,
    Country::TKL => COUNTRY_TKL_STR,
    Country::TON => COUNTRY_TON_STR,
    Country::TTO => COUNTRY_TTO_STR,
    Country::TUN => COUNTRY_TUN_STR,
    Country::TUR => COUNTRY_TUR_STR,
    Country::TKM => COUNTRY_TKM_STR,
    Country::TCA => COUNTRY_TCA_STR,
    Country::TUV => COUNTRY_TUV_STR,
    Country::UGA => COUNTRY_UGA_STR,
    Country::UKR => COUNTRY_UKR_STR,
    Country::ARE => COUNTRY_ARE_STR,
    Country::GBR => COUNTRY_GBR_STR,
    Country::USA => COUNTRY_USA_STR,
    Country::UMI => COUNTRY_UMI_STR,
    Country::URY => COUNTRY_URY_STR,
    Country::UZB => COUNTRY_UZB_STR,
    Country::VUT => COUNTRY_VUT_STR,
    Country::VEN => COUNTRY_VEN_STR,
    Country::VNM => COUNTRY_VNM_STR,
    Country::VGB => COUNTRY_VGB_STR,
    Country::VIR => COUNTRY_VIR_STR,
    Country::WLF => COUNTRY_WLF_STR,
    Country::ESH => COUNTRY_ESH_STR,
    Country::YEM => COUNTRY_YEM_STR,
    Country::ZMB => COUNTRY_ZMB_STR,
    Country::ZWE => COUNTRY_ZWE_STR,
}
}

/// Returns the enum given a string that might represent the variant's name.
pub fn country_from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Country>
{
    let variantstr = variantstr.as_ref();
match variantstr {
    COUNTRY_AFG_STR => Some(Country::AFG),
    COUNTRY_ALA_STR => Some(Country::ALA),
    COUNTRY_ALB_STR => Some(Country::ALB),
    COUNTRY_DZA_STR => Some(Country::DZA),
    COUNTRY_ASM_STR => Some(Country::ASM),
    COUNTRY_AND_STR => Some(Country::AND),
    COUNTRY_AGO_STR => Some(Country::AGO),
    COUNTRY_AIA_STR => Some(Country::AIA),
    COUNTRY_ATA_STR => Some(Country::ATA),
    COUNTRY_ATG_STR => Some(Country::ATG),
    COUNTRY_ARG_STR => Some(Country::ARG),
    COUNTRY_ARM_STR => Some(Country::ARM),
    COUNTRY_ABW_STR => Some(Country::ABW),
    COUNTRY_AUS_STR => Some(Country::AUS),
    COUNTRY_AUT_STR => Some(Country::AUT),
    COUNTRY_AZE_STR => Some(Country::AZE),
    COUNTRY_BHS_STR => Some(Country::BHS),
    COUNTRY_BHR_STR => Some(Country::BHR),
    COUNTRY_BGD_STR => Some(Country::BGD),
    COUNTRY_BRB_STR => Some(Country::BRB),
    COUNTRY_BLR_STR => Some(Country::BLR),
    COUNTRY_BEL_STR => Some(Country::BEL),
    COUNTRY_BLZ_STR => Some(Country::BLZ),
    COUNTRY_BEN_STR => Some(Country::BEN),
    COUNTRY_BMU_STR => Some(Country::BMU),
    COUNTRY_BTN_STR => Some(Country::BTN),
    COUNTRY_BOL_STR => Some(Country::BOL),
    COUNTRY_BES_STR => Some(Country::BES),
    COUNTRY_BIH_STR => Some(Country::BIH),
    COUNTRY_BWA_STR => Some(Country::BWA),
    COUNTRY_BVT_STR => Some(Country::BVT),
    COUNTRY_BRA_STR => Some(Country::BRA),
    COUNTRY_IOT_STR => Some(Country::IOT),
    COUNTRY_BRN_STR => Some(Country::BRN),
    COUNTRY_BGR_STR => Some(Country::BGR),
    COUNTRY_BFA_STR => Some(Country::BFA),
    COUNTRY_BDI_STR => Some(Country::BDI),
    COUNTRY_CPV_STR => Some(Country::CPV),
    COUNTRY_KHM_STR => Some(Country::KHM),
    COUNTRY_CMR_STR => Some(Country::CMR),
    COUNTRY_CAN_STR => Some(Country::CAN),
    COUNTRY_CYM_STR => Some(Country::CYM),
    COUNTRY_CAF_STR => Some(Country::CAF),
    COUNTRY_TCD_STR => Some(Country::TCD),
    COUNTRY_CHL_STR => Some(Country::CHL),
    COUNTRY_CHN_STR => Some(Country::CHN),
    COUNTRY_CXR_STR => Some(Country::CXR),
    COUNTRY_CCK_STR => Some(Country::CCK),
    COUNTRY_COL_STR => Some(Country::COL),
    COUNTRY_COM_STR => Some(Country::COM),
    COUNTRY_COG_STR => Some(Country::COG),
    COUNTRY_COD_STR => Some(Country::COD),
    COUNTRY_COK_STR => Some(Country::COK),
    COUNTRY_CRI_STR => Some(Country::CRI),
    COUNTRY_CIV_STR => Some(Country::CIV),
    COUNTRY_HRV_STR => Some(Country::HRV),
    COUNTRY_CUB_STR => Some(Country::CUB),
    COUNTRY_CUW_STR => Some(Country::CUW),
    COUNTRY_CYP_STR => Some(Country::CYP),
    COUNTRY_CZE_STR => Some(Country::CZE),
    COUNTRY_DNK_STR => Some(Country::DNK),
    COUNTRY_DJI_STR => Some(Country::DJI),
    COUNTRY_DMA_STR => Some(Country::DMA),
    COUNTRY_DOM_STR => Some(Country::DOM),
    COUNTRY_ECU_STR => Some(Country::ECU),
    COUNTRY_EGY_STR => Some(Country::EGY),
    COUNTRY_SLV_STR => Some(Country::SLV),
    COUNTRY_GNQ_STR => Some(Country::GNQ),
    COUNTRY_ERI_STR => Some(Country::ERI),
    COUNTRY_EST_STR => Some(Country::EST),
    COUNTRY_SWZ_STR => Some(Country::SWZ),
    COUNTRY_ETH_STR => Some(Country::ETH),
    COUNTRY_FLK_STR => Some(Country::FLK),
    COUNTRY_FRO_STR => Some(Country::FRO),
    COUNTRY_FJI_STR => Some(Country::FJI),
    COUNTRY_FIN_STR => Some(Country::FIN),
    COUNTRY_FRA_STR => Some(Country::FRA),
    COUNTRY_GUF_STR => Some(Country::GUF),
    COUNTRY_PYF_STR => Some(Country::PYF),
    COUNTRY_ATF_STR => Some(Country::ATF),
    COUNTRY_GAB_STR => Some(Country::GAB),
    COUNTRY_GMB_STR => Some(Country::GMB),
    COUNTRY_GEO_STR => Some(Country::GEO),
    COUNTRY_DEU_STR => Some(Country::DEU),
    COUNTRY_GHA_STR => Some(Country::GHA),
    COUNTRY_GIB_STR => Some(Country::GIB),
    COUNTRY_GRC_STR => Some(Country::GRC),
    COUNTRY_GRL_STR => Some(Country::GRL),
    COUNTRY_GRD_STR => Some(Country::GRD),
    COUNTRY_GLP_STR => Some(Country::GLP),
    COUNTRY_GUM_STR => Some(Country::GUM),
    COUNTRY_GTM_STR => Some(Country::GTM),
    COUNTRY_GGY_STR => Some(Country::GGY),
    COUNTRY_GIN_STR => Some(Country::GIN),
    COUNTRY_GNB_STR => Some(Country::GNB),
    COUNTRY_GUY_STR => Some(Country::GUY),
    COUNTRY_HTI_STR => Some(Country::HTI),
    COUNTRY_HMD_STR => Some(Country::HMD),
    COUNTRY_VAT_STR => Some(Country::VAT),
    COUNTRY_HND_STR => Some(Country::HND),
    COUNTRY_HKG_STR => Some(Country::HKG),
    COUNTRY_HUN_STR => Some(Country::HUN),
    COUNTRY_ISL_STR => Some(Country::ISL),
    COUNTRY_IND_STR => Some(Country::IND),
    COUNTRY_IDN_STR => Some(Country::IDN),
    COUNTRY_IRN_STR => Some(Country::IRN),
    COUNTRY_IRQ_STR => Some(Country::IRQ),
    COUNTRY_IRL_STR => Some(Country::IRL),
    COUNTRY_IMN_STR => Some(Country::IMN),
    COUNTRY_ISR_STR => Some(Country::ISR),
    COUNTRY_ITA_STR => Some(Country::ITA),
    COUNTRY_JAM_STR => Some(Country::JAM),
    COUNTRY_JPN_STR => Some(Country::JPN),
    COUNTRY_JEY_STR => Some(Country::JEY),
    COUNTRY_JOR_STR => Some(Country::JOR),
    COUNTRY_KAZ_STR => Some(Country::KAZ),
    COUNTRY_KEN_STR => Some(Country::KEN),
    COUNTRY_KIR_STR => Some(Country::KIR),
    COUNTRY_PRK_STR => Some(Country::PRK),
    COUNTRY_KOR_STR => Some(Country::KOR),
    COUNTRY_KWT_STR => Some(Country::KWT),
    COUNTRY_KGZ_STR => Some(Country::KGZ),
    COUNTRY_LAO_STR => Some(Country::LAO),
    COUNTRY_LVA_STR => Some(Country::LVA),
    COUNTRY_LBN_STR => Some(Country::LBN),
    COUNTRY_LSO_STR => Some(Country::LSO),
    COUNTRY_LBR_STR => Some(Country::LBR),
    COUNTRY_LBY_STR => Some(Country::LBY),
    COUNTRY_LIE_STR => Some(Country::LIE),
    COUNTRY_LTU_STR => Some(Country::LTU),
    COUNTRY_LUX_STR => Some(Country::LUX),
    COUNTRY_MAC_STR => Some(Country::MAC),
    COUNTRY_MDG_STR => Some(Country::MDG),
    COUNTRY_MWI_STR => Some(Country::MWI),
    COUNTRY_MYS_STR => Some(Country::MYS),
    COUNTRY_MDV_STR => Some(Country::MDV),
    COUNTRY_MLI_STR => Some(Country::MLI),
    COUNTRY_MLT_STR => Some(Country::MLT),
    COUNTRY_MHL_STR => Some(Country::MHL),
    COUNTRY_MTQ_STR => Some(Country::MTQ),
    COUNTRY_MRT_STR => Some(Country::MRT),
    COUNTRY_MUS_STR => Some(Country::MUS),
    COUNTRY_MYT_STR => Some(Country::MYT),
    COUNTRY_MEX_STR => Some(Country::MEX),
    COUNTRY_FSM_STR => Some(Country::FSM),
    COUNTRY_MDA_STR => Some(Country::MDA),
    COUNTRY_MCO_STR => Some(Country::MCO),
    COUNTRY_MNG_STR => Some(Country::MNG),
    COUNTRY_MNE_STR => Some(Country::MNE),
    COUNTRY_MSR_STR => Some(Country::MSR),
    COUNTRY_MAR_STR => Some(Country::MAR),
    COUNTRY_MOZ_STR => Some(Country::MOZ),
    COUNTRY_MMR_STR => Some(Country::MMR),
    COUNTRY_NAM_STR => Some(Country::NAM),
    COUNTRY_NRU_STR => Some(Country::NRU),
    COUNTRY_NPL_STR => Some(Country::NPL),
    COUNTRY_NLD_STR => Some(Country::NLD),
    COUNTRY_NCL_STR => Some(Country::NCL),
    COUNTRY_NZL_STR => Some(Country::NZL),
    COUNTRY_NIC_STR => Some(Country::NIC),
    COUNTRY_NER_STR => Some(Country::NER),
    COUNTRY_NGA_STR => Some(Country::NGA),
    COUNTRY_NIU_STR => Some(Country::NIU),
    COUNTRY_NFK_STR => Some(Country::NFK),
    COUNTRY_MKD_STR => Some(Country::MKD),
    COUNTRY_MNP_STR => Some(Country::MNP),
    COUNTRY_NOR_STR => Some(Country::NOR),
    COUNTRY_OMN_STR => Some(Country::OMN),
    COUNTRY_PAK_STR => Some(Country::PAK),
    COUNTRY_PLW_STR => Some(Country::PLW),
    COUNTRY_PSE_STR => Some(Country::PSE),
    COUNTRY_PAN_STR => Some(Country::PAN),
    COUNTRY_PNG_STR => Some(Country::PNG),
    COUNTRY_PRY_STR => Some(Country::PRY),
    COUNTRY_PER_STR => Some(Country::PER),
    COUNTRY_PHL_STR => Some(Country::PHL),
    COUNTRY_PCN_STR => Some(Country::PCN),
    COUNTRY_POL_STR => Some(Country::POL),
    COUNTRY_PRT_STR => Some(Country::PRT),
    COUNTRY_PRI_STR => Some(Country::PRI),
    COUNTRY_QAT_STR => Some(Country::QAT),
    COUNTRY_REU_STR => Some(Country::REU),
    COUNTRY_ROU_STR => Some(Country::ROU),
    COUNTRY_RUS_STR => Some(Country::RUS),
    COUNTRY_RWA_STR => Some(Country::RWA),
    COUNTRY_BLM_STR => Some(Country::BLM),
    COUNTRY_SHN_STR => Some(Country::SHN),
    COUNTRY_KNA_STR => Some(Country::KNA),
    COUNTRY_LCA_STR => Some(Country::LCA),
    COUNTRY_MAF_STR => Some(Country::MAF),
    COUNTRY_SPM_STR => Some(Country::SPM),
    COUNTRY_VCT_STR => Some(Country::VCT),
    COUNTRY_WSM_STR => Some(Country::WSM),
    COUNTRY_SMR_STR => Some(Country::SMR),
    COUNTRY_STP_STR => Some(Country::STP),
    COUNTRY_SAU_STR => Some(Country::SAU),
    COUNTRY_SEN_STR => Some(Country::SEN),
    COUNTRY_SRB_STR => Some(Country::SRB),
    COUNTRY_SYC_STR => Some(Country::SYC),
    COUNTRY_SLE_STR => Some(Country::SLE),
    COUNTRY_SGP_STR => Some(Country::SGP),
    COUNTRY_SXM_STR => Some(Country::SXM),
    COUNTRY_SVK_STR => Some(Country::SVK),
    COUNTRY_SVN_STR => Some(Country::SVN),
    COUNTRY_SLB_STR => Some(Country::SLB),
    COUNTRY_SOM_STR => Some(Country::SOM),
    COUNTRY_ZAF_STR => Some(Country::ZAF),
    COUNTRY_SGS_STR => Some(Country::SGS),
    COUNTRY_SSD_STR => Some(Country::SSD),
    COUNTRY_ESP_STR => Some(Country::ESP),
    COUNTRY_LKA_STR => Some(Country::LKA),
    COUNTRY_SDN_STR => Some(Country::SDN),
    COUNTRY_SUR_STR => Some(Country::SUR),
    COUNTRY_SJM_STR => Some(Country::SJM),
    COUNTRY_SWE_STR => Some(Country::SWE),
    COUNTRY_CHE_STR => Some(Country::CHE),
    COUNTRY_SYR_STR => Some(Country::SYR),
    COUNTRY_TWN_STR => Some(Country::TWN),
    COUNTRY_TJK_STR => Some(Country::TJK),
    COUNTRY_TZA_STR => Some(Country::TZA),
    COUNTRY_THA_STR => Some(Country::THA),
    COUNTRY_TLS_STR => Some(Country::TLS),
    COUNTRY_TGO_STR => Some(Country::TGO),
    COUNTRY_TKL_STR => Some(Country::TKL),
    COUNTRY_TON_STR => Some(Country::TON),
    COUNTRY_TTO_STR => Some(Country::TTO),
    COUNTRY_TUN_STR => Some(Country::TUN),
    COUNTRY_TUR_STR => Some(Country::TUR),
    COUNTRY_TKM_STR => Some(Country::TKM),
    COUNTRY_TCA_STR => Some(Country::TCA),
    COUNTRY_TUV_STR => Some(Country::TUV),
    COUNTRY_UGA_STR => Some(Country::UGA),
    COUNTRY_UKR_STR => Some(Country::UKR),
    COUNTRY_ARE_STR => Some(Country::ARE),
    COUNTRY_GBR_STR => Some(Country::GBR),
    COUNTRY_USA_STR => Some(Country::USA),
    COUNTRY_UMI_STR => Some(Country::UMI),
    COUNTRY_URY_STR => Some(Country::URY),
    COUNTRY_UZB_STR => Some(Country::UZB),
    COUNTRY_VUT_STR => Some(Country::VUT),
    COUNTRY_VEN_STR => Some(Country::VEN),
    COUNTRY_VNM_STR => Some(Country::VNM),
    COUNTRY_VGB_STR => Some(Country::VGB),
    COUNTRY_VIR_STR => Some(Country::VIR),
    COUNTRY_WLF_STR => Some(Country::WLF),
    COUNTRY_ESH_STR => Some(Country::ESH),
    COUNTRY_YEM_STR => Some(Country::YEM),
    COUNTRY_ZMB_STR => Some(Country::ZMB),
    COUNTRY_ZWE_STR => Some(Country::ZWE),
    _ => Option::None,
}
}
/// Constants for `country_code`
const COUNTRY_CODE_AFG: usize = 4;
const COUNTRY_CODE_ALA: usize = 248;
const COUNTRY_CODE_ALB: usize = 8;
const COUNTRY_CODE_DZA: usize = 12;
const COUNTRY_CODE_ASM: usize = 16;
const COUNTRY_CODE_AND: usize = 20;
const COUNTRY_CODE_AGO: usize = 24;
const COUNTRY_CODE_AIA: usize = 660;
const COUNTRY_CODE_ATA: usize = 10;
const COUNTRY_CODE_ATG: usize = 28;
const COUNTRY_CODE_ARG: usize = 32;
const COUNTRY_CODE_ARM: usize = 51;
const COUNTRY_CODE_ABW: usize = 533;
const COUNTRY_CODE_AUS: usize = 36;
const COUNTRY_CODE_AUT: usize = 40;
const COUNTRY_CODE_AZE: usize = 31;
const COUNTRY_CODE_BHS: usize = 44;
const COUNTRY_CODE_BHR: usize = 48;
const COUNTRY_CODE_BGD: usize = 50;
const COUNTRY_CODE_BRB: usize = 52;
const COUNTRY_CODE_BLR: usize = 112;
const COUNTRY_CODE_BEL: usize = 56;
const COUNTRY_CODE_BLZ: usize = 84;
const COUNTRY_CODE_BEN: usize = 204;
const COUNTRY_CODE_BMU: usize = 60;
const COUNTRY_CODE_BTN: usize = 64;
const COUNTRY_CODE_BOL: usize = 68;
const COUNTRY_CODE_BES: usize = 535;
const COUNTRY_CODE_BIH: usize = 70;
const COUNTRY_CODE_BWA: usize = 72;
const COUNTRY_CODE_BVT: usize = 74;
const COUNTRY_CODE_BRA: usize = 76;
const COUNTRY_CODE_IOT: usize = 86;
const COUNTRY_CODE_BRN: usize = 96;
const COUNTRY_CODE_BGR: usize = 100;
const COUNTRY_CODE_BFA: usize = 854;
const COUNTRY_CODE_BDI: usize = 108;
const COUNTRY_CODE_CPV: usize = 132;
const COUNTRY_CODE_KHM: usize = 116;
const COUNTRY_CODE_CMR: usize = 120;
const COUNTRY_CODE_CAN: usize = 124;
const COUNTRY_CODE_CYM: usize = 136;
const COUNTRY_CODE_CAF: usize = 140;
const COUNTRY_CODE_TCD: usize = 148;
const COUNTRY_CODE_CHL: usize = 152;
const COUNTRY_CODE_CHN: usize = 156;
const COUNTRY_CODE_CXR: usize = 162;
const COUNTRY_CODE_CCK: usize = 166;
const COUNTRY_CODE_COL: usize = 170;
const COUNTRY_CODE_COM: usize = 174;
const COUNTRY_CODE_COG: usize = 178;
const COUNTRY_CODE_COD: usize = 180;
const COUNTRY_CODE_COK: usize = 184;
const COUNTRY_CODE_CRI: usize = 188;
const COUNTRY_CODE_CIV: usize = 384;
const COUNTRY_CODE_HRV: usize = 191;
const COUNTRY_CODE_CUB: usize = 192;
const COUNTRY_CODE_CUW: usize = 531;
const COUNTRY_CODE_CYP: usize = 196;
const COUNTRY_CODE_CZE: usize = 203;
const COUNTRY_CODE_DNK: usize = 208;
const COUNTRY_CODE_DJI: usize = 262;
const COUNTRY_CODE_DMA: usize = 212;
const COUNTRY_CODE_DOM: usize = 214;
const COUNTRY_CODE_ECU: usize = 218;
const COUNTRY_CODE_EGY: usize = 818;
const COUNTRY_CODE_SLV: usize = 222;
const COUNTRY_CODE_GNQ: usize = 226;
const COUNTRY_CODE_ERI: usize = 232;
const COUNTRY_CODE_EST: usize = 233;
const COUNTRY_CODE_SWZ: usize = 748;
const COUNTRY_CODE_ETH: usize = 231;
const COUNTRY_CODE_FLK: usize = 238;
const COUNTRY_CODE_FRO: usize = 234;
const COUNTRY_CODE_FJI: usize = 242;
const COUNTRY_CODE_FIN: usize = 246;
const COUNTRY_CODE_FRA: usize = 250;
const COUNTRY_CODE_GUF: usize = 254;
const COUNTRY_CODE_PYF: usize = 258;
const COUNTRY_CODE_ATF: usize = 260;
const COUNTRY_CODE_GAB: usize = 266;
const COUNTRY_CODE_GMB: usize = 270;
const COUNTRY_CODE_GEO: usize = 268;
const COUNTRY_CODE_DEU: usize = 276;
const COUNTRY_CODE_GHA: usize = 288;
const COUNTRY_CODE_GIB: usize = 292;
const COUNTRY_CODE_GRC: usize = 300;
const COUNTRY_CODE_GRL: usize = 304;
const COUNTRY_CODE_GRD: usize = 308;
const COUNTRY_CODE_GLP: usize = 312;
const COUNTRY_CODE_GUM: usize = 316;
const COUNTRY_CODE_GTM: usize = 320;
const COUNTRY_CODE_GGY: usize = 831;
const COUNTRY_CODE_GIN: usize = 324;
const COUNTRY_CODE_GNB: usize = 624;
const COUNTRY_CODE_GUY: usize = 328;
const COUNTRY_CODE_HTI: usize = 332;
const COUNTRY_CODE_HMD: usize = 334;
const COUNTRY_CODE_VAT: usize = 336;
const COUNTRY_CODE_HND: usize = 340;
const COUNTRY_CODE_HKG: usize = 344;
const COUNTRY_CODE_HUN: usize = 348;
const COUNTRY_CODE_ISL: usize = 352;
const COUNTRY_CODE_IND: usize = 356;
const COUNTRY_CODE_IDN: usize = 360;
const COUNTRY_CODE_IRN: usize = 364;
const COUNTRY_CODE_IRQ: usize = 368;
const COUNTRY_CODE_IRL: usize = 372;
const COUNTRY_CODE_IMN: usize = 833;
const COUNTRY_CODE_ISR: usize = 376;
const COUNTRY_CODE_ITA: usize = 380;
const COUNTRY_CODE_JAM: usize = 388;
const COUNTRY_CODE_JPN: usize = 392;
const COUNTRY_CODE_JEY: usize = 832;
const COUNTRY_CODE_JOR: usize = 400;
const COUNTRY_CODE_KAZ: usize = 398;
const COUNTRY_CODE_KEN: usize = 404;
const COUNTRY_CODE_KIR: usize = 296;
const COUNTRY_CODE_PRK: usize = 408;
const COUNTRY_CODE_KOR: usize = 410;
const COUNTRY_CODE_KWT: usize = 414;
const COUNTRY_CODE_KGZ: usize = 417;
const COUNTRY_CODE_LAO: usize = 418;
const COUNTRY_CODE_LVA: usize = 428;
const COUNTRY_CODE_LBN: usize = 422;
const COUNTRY_CODE_LSO: usize = 426;
const COUNTRY_CODE_LBR: usize = 430;
const COUNTRY_CODE_LBY: usize = 434;
const COUNTRY_CODE_LIE: usize = 438;
const COUNTRY_CODE_LTU: usize = 440;
const COUNTRY_CODE_LUX: usize = 442;
const COUNTRY_CODE_MAC: usize = 446;
const COUNTRY_CODE_MDG: usize = 450;
const COUNTRY_CODE_MWI: usize = 454;
const COUNTRY_CODE_MYS: usize = 458;
const COUNTRY_CODE_MDV: usize = 462;
const COUNTRY_CODE_MLI: usize = 466;
const COUNTRY_CODE_MLT: usize = 470;
const COUNTRY_CODE_MHL: usize = 584;
const COUNTRY_CODE_MTQ: usize = 474;
const COUNTRY_CODE_MRT: usize = 478;
const COUNTRY_CODE_MUS: usize = 480;
const COUNTRY_CODE_MYT: usize = 175;
const COUNTRY_CODE_MEX: usize = 484;
const COUNTRY_CODE_FSM: usize = 583;
const COUNTRY_CODE_MDA: usize = 498;
const COUNTRY_CODE_MCO: usize = 492;
const COUNTRY_CODE_MNG: usize = 496;
const COUNTRY_CODE_MNE: usize = 499;
const COUNTRY_CODE_MSR: usize = 500;
const COUNTRY_CODE_MAR: usize = 504;
const COUNTRY_CODE_MOZ: usize = 508;
const COUNTRY_CODE_MMR: usize = 104;
const COUNTRY_CODE_NAM: usize = 516;
const COUNTRY_CODE_NRU: usize = 520;
const COUNTRY_CODE_NPL: usize = 524;
const COUNTRY_CODE_NLD: usize = 528;
const COUNTRY_CODE_NCL: usize = 540;
const COUNTRY_CODE_NZL: usize = 554;
const COUNTRY_CODE_NIC: usize = 558;
const COUNTRY_CODE_NER: usize = 562;
const COUNTRY_CODE_NGA: usize = 566;
const COUNTRY_CODE_NIU: usize = 570;
const COUNTRY_CODE_NFK: usize = 574;
const COUNTRY_CODE_MKD: usize = 807;
const COUNTRY_CODE_MNP: usize = 580;
const COUNTRY_CODE_NOR: usize = 578;
const COUNTRY_CODE_OMN: usize = 512;
const COUNTRY_CODE_PAK: usize = 586;
const COUNTRY_CODE_PLW: usize = 585;
const COUNTRY_CODE_PSE: usize = 275;
const COUNTRY_CODE_PAN: usize = 591;
const COUNTRY_CODE_PNG: usize = 598;
const COUNTRY_CODE_PRY: usize = 600;
const COUNTRY_CODE_PER: usize = 604;
const COUNTRY_CODE_PHL: usize = 608;
const COUNTRY_CODE_PCN: usize = 612;
const COUNTRY_CODE_POL: usize = 616;
const COUNTRY_CODE_PRT: usize = 620;
const COUNTRY_CODE_PRI: usize = 630;
const COUNTRY_CODE_QAT: usize = 634;
const COUNTRY_CODE_REU: usize = 638;
const COUNTRY_CODE_ROU: usize = 642;
const COUNTRY_CODE_RUS: usize = 643;
const COUNTRY_CODE_RWA: usize = 646;
const COUNTRY_CODE_BLM: usize = 652;
const COUNTRY_CODE_SHN: usize = 654;
const COUNTRY_CODE_KNA: usize = 659;
const COUNTRY_CODE_LCA: usize = 662;
const COUNTRY_CODE_MAF: usize = 663;
const COUNTRY_CODE_SPM: usize = 666;
const COUNTRY_CODE_VCT: usize = 670;
const COUNTRY_CODE_WSM: usize = 882;
const COUNTRY_CODE_SMR: usize = 674;
const COUNTRY_CODE_STP: usize = 678;
const COUNTRY_CODE_SAU: usize = 682;
const COUNTRY_CODE_SEN: usize = 686;
const COUNTRY_CODE_SRB: usize = 688;
const COUNTRY_CODE_SYC: usize = 690;
const COUNTRY_CODE_SLE: usize = 694;
const COUNTRY_CODE_SGP: usize = 702;
const COUNTRY_CODE_SXM: usize = 534;
const COUNTRY_CODE_SVK: usize = 703;
const COUNTRY_CODE_SVN: usize = 705;
const COUNTRY_CODE_SLB: usize = 90;
const COUNTRY_CODE_SOM: usize = 706;
const COUNTRY_CODE_ZAF: usize = 710;
const COUNTRY_CODE_SGS: usize = 239;
const COUNTRY_CODE_SSD: usize = 728;
const COUNTRY_CODE_ESP: usize = 724;
const COUNTRY_CODE_LKA: usize = 144;
const COUNTRY_CODE_SDN: usize = 729;
const COUNTRY_CODE_SUR: usize = 740;
const COUNTRY_CODE_SJM: usize = 744;
const COUNTRY_CODE_SWE: usize = 752;
const COUNTRY_CODE_CHE: usize = 756;
const COUNTRY_CODE_SYR: usize = 760;
const COUNTRY_CODE_TWN: usize = 158;
const COUNTRY_CODE_TJK: usize = 762;
const COUNTRY_CODE_TZA: usize = 834;
const COUNTRY_CODE_THA: usize = 764;
const COUNTRY_CODE_TLS: usize = 626;
const COUNTRY_CODE_TGO: usize = 768;
const COUNTRY_CODE_TKL: usize = 772;
const COUNTRY_CODE_TON: usize = 776;
const COUNTRY_CODE_TTO: usize = 780;
const COUNTRY_CODE_TUN: usize = 788;
const COUNTRY_CODE_TUR: usize = 792;
const COUNTRY_CODE_TKM: usize = 795;
const COUNTRY_CODE_TCA: usize = 796;
const COUNTRY_CODE_TUV: usize = 798;
const COUNTRY_CODE_UGA: usize = 800;
const COUNTRY_CODE_UKR: usize = 804;
const COUNTRY_CODE_ARE: usize = 784;
const COUNTRY_CODE_GBR: usize = 826;
const COUNTRY_CODE_USA: usize = 840;
const COUNTRY_CODE_UMI: usize = 581;
const COUNTRY_CODE_URY: usize = 858;
const COUNTRY_CODE_UZB: usize = 860;
const COUNTRY_CODE_VUT: usize = 548;
const COUNTRY_CODE_VEN: usize = 862;
const COUNTRY_CODE_VNM: usize = 704;
const COUNTRY_CODE_VGB: usize = 92;
const COUNTRY_CODE_VIR: usize = 850;
const COUNTRY_CODE_WLF: usize = 876;
const COUNTRY_CODE_ESH: usize = 732;
const COUNTRY_CODE_YEM: usize = 887;
const COUNTRY_CODE_ZMB: usize = 894;
const COUNTRY_CODE_ZWE: usize = 716;

/// Function to convert from Country to country_code
pub const fn country_as_country_code(country: &Country) -> usize
{
match country {
    Country::AFG => COUNTRY_CODE_AFG,
    Country::ALA => COUNTRY_CODE_ALA,
    Country::ALB => COUNTRY_CODE_ALB,
    Country::DZA => COUNTRY_CODE_DZA,
    Country::ASM => COUNTRY_CODE_ASM,
    Country::AND => COUNTRY_CODE_AND,
    Country::AGO => COUNTRY_CODE_AGO,
    Country::AIA => COUNTRY_CODE_AIA,
    Country::ATA => COUNTRY_CODE_ATA,
    Country::ATG => COUNTRY_CODE_ATG,
    Country::ARG => COUNTRY_CODE_ARG,
    Country::ARM => COUNTRY_CODE_ARM,
    Country::ABW => COUNTRY_CODE_ABW,
    Country::AUS => COUNTRY_CODE_AUS,
    Country::AUT => COUNTRY_CODE_AUT,
    Country::AZE => COUNTRY_CODE_AZE,
    Country::BHS => COUNTRY_CODE_BHS,
    Country::BHR => COUNTRY_CODE_BHR,
    Country::BGD => COUNTRY_CODE_BGD,
    Country::BRB => COUNTRY_CODE_BRB,
    Country::BLR => COUNTRY_CODE_BLR,
    Country::BEL => COUNTRY_CODE_BEL,
    Country::BLZ => COUNTRY_CODE_BLZ,
    Country::BEN => COUNTRY_CODE_BEN,
    Country::BMU => COUNTRY_CODE_BMU,
    Country::BTN => COUNTRY_CODE_BTN,
    Country::BOL => COUNTRY_CODE_BOL,
    Country::BES => COUNTRY_CODE_BES,
    Country::BIH => COUNTRY_CODE_BIH,
    Country::BWA => COUNTRY_CODE_BWA,
    Country::BVT => COUNTRY_CODE_BVT,
    Country::BRA => COUNTRY_CODE_BRA,
    Country::IOT => COUNTRY_CODE_IOT,
    Country::BRN => COUNTRY_CODE_BRN,
    Country::BGR => COUNTRY_CODE_BGR,
    Country::BFA => COUNTRY_CODE_BFA,
    Country::BDI => COUNTRY_CODE_BDI,
    Country::CPV => COUNTRY_CODE_CPV,
    Country::KHM => COUNTRY_CODE_KHM,
    Country::CMR => COUNTRY_CODE_CMR,
    Country::CAN => COUNTRY_CODE_CAN,
    Country::CYM => COUNTRY_CODE_CYM,
    Country::CAF => COUNTRY_CODE_CAF,
    Country::TCD => COUNTRY_CODE_TCD,
    Country::CHL => COUNTRY_CODE_CHL,
    Country::CHN => COUNTRY_CODE_CHN,
    Country::CXR => COUNTRY_CODE_CXR,
    Country::CCK => COUNTRY_CODE_CCK,
    Country::COL => COUNTRY_CODE_COL,
    Country::COM => COUNTRY_CODE_COM,
    Country::COG => COUNTRY_CODE_COG,
    Country::COD => COUNTRY_CODE_COD,
    Country::COK => COUNTRY_CODE_COK,
    Country::CRI => COUNTRY_CODE_CRI,
    Country::CIV => COUNTRY_CODE_CIV,
    Country::HRV => COUNTRY_CODE_HRV,
    Country::CUB => COUNTRY_CODE_CUB,
    Country::CUW => COUNTRY_CODE_CUW,
    Country::CYP => COUNTRY_CODE_CYP,
    Country::CZE => COUNTRY_CODE_CZE,
    Country::DNK => COUNTRY_CODE_DNK,
    Country::DJI => COUNTRY_CODE_DJI,
    Country::DMA => COUNTRY_CODE_DMA,
    Country::DOM => COUNTRY_CODE_DOM,
    Country::ECU => COUNTRY_CODE_ECU,
    Country::EGY => COUNTRY_CODE_EGY,
    Country::SLV => COUNTRY_CODE_SLV,
    Country::GNQ => COUNTRY_CODE_GNQ,
    Country::ERI => COUNTRY_CODE_ERI,
    Country::EST => COUNTRY_CODE_EST,
    Country::SWZ => COUNTRY_CODE_SWZ,
    Country::ETH => COUNTRY_CODE_ETH,
    Country::FLK => COUNTRY_CODE_FLK,
    Country::FRO => COUNTRY_CODE_FRO,
    Country::FJI => COUNTRY_CODE_FJI,
    Country::FIN => COUNTRY_CODE_FIN,
    Country::FRA => COUNTRY_CODE_FRA,
    Country::GUF => COUNTRY_CODE_GUF,
    Country::PYF => COUNTRY_CODE_PYF,
    Country::ATF => COUNTRY_CODE_ATF,
    Country::GAB => COUNTRY_CODE_GAB,
    Country::GMB => COUNTRY_CODE_GMB,
    Country::GEO => COUNTRY_CODE_GEO,
    Country::DEU => COUNTRY_CODE_DEU,
    Country::GHA => COUNTRY_CODE_GHA,
    Country::GIB => COUNTRY_CODE_GIB,
    Country::GRC => COUNTRY_CODE_GRC,
    Country::GRL => COUNTRY_CODE_GRL,
    Country::GRD => COUNTRY_CODE_GRD,
    Country::GLP => COUNTRY_CODE_GLP,
    Country::GUM => COUNTRY_CODE_GUM,
    Country::GTM => COUNTRY_CODE_GTM,
    Country::GGY => COUNTRY_CODE_GGY,
    Country::GIN => COUNTRY_CODE_GIN,
    Country::GNB => COUNTRY_CODE_GNB,
    Country::GUY => COUNTRY_CODE_GUY,
    Country::HTI => COUNTRY_CODE_HTI,
    Country::HMD => COUNTRY_CODE_HMD,
    Country::VAT => COUNTRY_CODE_VAT,
    Country::HND => COUNTRY_CODE_HND,
    Country::HKG => COUNTRY_CODE_HKG,
    Country::HUN => COUNTRY_CODE_HUN,
    Country::ISL => COUNTRY_CODE_ISL,
    Country::IND => COUNTRY_CODE_IND,
    Country::IDN => COUNTRY_CODE_IDN,
    Country::IRN => COUNTRY_CODE_IRN,
    Country::IRQ => COUNTRY_CODE_IRQ,
    Country::IRL => COUNTRY_CODE_IRL,
    Country::IMN => COUNTRY_CODE_IMN,
    Country::ISR => COUNTRY_CODE_ISR,
    Country::ITA => COUNTRY_CODE_ITA,
    Country::JAM => COUNTRY_CODE_JAM,
    Country::JPN => COUNTRY_CODE_JPN,
    Country::JEY => COUNTRY_CODE_JEY,
    Country::JOR => COUNTRY_CODE_JOR,
    Country::KAZ => COUNTRY_CODE_KAZ,
    Country::KEN => COUNTRY_CODE_KEN,
    Country::KIR => COUNTRY_CODE_KIR,
    Country::PRK => COUNTRY_CODE_PRK,
    Country::KOR => COUNTRY_CODE_KOR,
    Country::KWT => COUNTRY_CODE_KWT,
    Country::KGZ => COUNTRY_CODE_KGZ,
    Country::LAO => COUNTRY_CODE_LAO,
    Country::LVA => COUNTRY_CODE_LVA,
    Country::LBN => COUNTRY_CODE_LBN,
    Country::LSO => COUNTRY_CODE_LSO,
    Country::LBR => COUNTRY_CODE_LBR,
    Country::LBY => COUNTRY_CODE_LBY,
    Country::LIE => COUNTRY_CODE_LIE,
    Country::LTU => COUNTRY_CODE_LTU,
    Country::LUX => COUNTRY_CODE_LUX,
    Country::MAC => COUNTRY_CODE_MAC,
    Country::MDG => COUNTRY_CODE_MDG,
    Country::MWI => COUNTRY_CODE_MWI,
    Country::MYS => COUNTRY_CODE_MYS,
    Country::MDV => COUNTRY_CODE_MDV,
    Country::MLI => COUNTRY_CODE_MLI,
    Country::MLT => COUNTRY_CODE_MLT,
    Country::MHL => COUNTRY_CODE_MHL,
    Country::MTQ => COUNTRY_CODE_MTQ,
    Country::MRT => COUNTRY_CODE_MRT,
    Country::MUS => COUNTRY_CODE_MUS,
    Country::MYT => COUNTRY_CODE_MYT,
    Country::MEX => COUNTRY_CODE_MEX,
    Country::FSM => COUNTRY_CODE_FSM,
    Country::MDA => COUNTRY_CODE_MDA,
    Country::MCO => COUNTRY_CODE_MCO,
    Country::MNG => COUNTRY_CODE_MNG,
    Country::MNE => COUNTRY_CODE_MNE,
    Country::MSR => COUNTRY_CODE_MSR,
    Country::MAR => COUNTRY_CODE_MAR,
    Country::MOZ => COUNTRY_CODE_MOZ,
    Country::MMR => COUNTRY_CODE_MMR,
    Country::NAM => COUNTRY_CODE_NAM,
    Country::NRU => COUNTRY_CODE_NRU,
    Country::NPL => COUNTRY_CODE_NPL,
    Country::NLD => COUNTRY_CODE_NLD,
    Country::NCL => COUNTRY_CODE_NCL,
    Country::NZL => COUNTRY_CODE_NZL,
    Country::NIC => COUNTRY_CODE_NIC,
    Country::NER => COUNTRY_CODE_NER,
    Country::NGA => COUNTRY_CODE_NGA,
    Country::NIU => COUNTRY_CODE_NIU,
    Country::NFK => COUNTRY_CODE_NFK,
    Country::MKD => COUNTRY_CODE_MKD,
    Country::MNP => COUNTRY_CODE_MNP,
    Country::NOR => COUNTRY_CODE_NOR,
    Country::OMN => COUNTRY_CODE_OMN,
    Country::PAK => COUNTRY_CODE_PAK,
    Country::PLW => COUNTRY_CODE_PLW,
    Country::PSE => COUNTRY_CODE_PSE,
    Country::PAN => COUNTRY_CODE_PAN,
    Country::PNG => COUNTRY_CODE_PNG,
    Country::PRY => COUNTRY_CODE_PRY,
    Country::PER => COUNTRY_CODE_PER,
    Country::PHL => COUNTRY_CODE_PHL,
    Country::PCN => COUNTRY_CODE_PCN,
    Country::POL => COUNTRY_CODE_POL,
    Country::PRT => COUNTRY_CODE_PRT,
    Country::PRI => COUNTRY_CODE_PRI,
    Country::QAT => COUNTRY_CODE_QAT,
    Country::REU => COUNTRY_CODE_REU,
    Country::ROU => COUNTRY_CODE_ROU,
    Country::RUS => COUNTRY_CODE_RUS,
    Country::RWA => COUNTRY_CODE_RWA,
    Country::BLM => COUNTRY_CODE_BLM,
    Country::SHN => COUNTRY_CODE_SHN,
    Country::KNA => COUNTRY_CODE_KNA,
    Country::LCA => COUNTRY_CODE_LCA,
    Country::MAF => COUNTRY_CODE_MAF,
    Country::SPM => COUNTRY_CODE_SPM,
    Country::VCT => COUNTRY_CODE_VCT,
    Country::WSM => COUNTRY_CODE_WSM,
    Country::SMR => COUNTRY_CODE_SMR,
    Country::STP => COUNTRY_CODE_STP,
    Country::SAU => COUNTRY_CODE_SAU,
    Country::SEN => COUNTRY_CODE_SEN,
    Country::SRB => COUNTRY_CODE_SRB,
    Country::SYC => COUNTRY_CODE_SYC,
    Country::SLE => COUNTRY_CODE_SLE,
    Country::SGP => COUNTRY_CODE_SGP,
    Country::SXM => COUNTRY_CODE_SXM,
    Country::SVK => COUNTRY_CODE_SVK,
    Country::SVN => COUNTRY_CODE_SVN,
    Country::SLB => COUNTRY_CODE_SLB,
    Country::SOM => COUNTRY_CODE_SOM,
    Country::ZAF => COUNTRY_CODE_ZAF,
    Country::SGS => COUNTRY_CODE_SGS,
    Country::SSD => COUNTRY_CODE_SSD,
    Country::ESP => COUNTRY_CODE_ESP,
    Country::LKA => COUNTRY_CODE_LKA,
    Country::SDN => COUNTRY_CODE_SDN,
    Country::SUR => COUNTRY_CODE_SUR,
    Country::SJM => COUNTRY_CODE_SJM,
    Country::SWE => COUNTRY_CODE_SWE,
    Country::CHE => COUNTRY_CODE_CHE,
    Country::SYR => COUNTRY_CODE_SYR,
    Country::TWN => COUNTRY_CODE_TWN,
    Country::TJK => COUNTRY_CODE_TJK,
    Country::TZA => COUNTRY_CODE_TZA,
    Country::THA => COUNTRY_CODE_THA,
    Country::TLS => COUNTRY_CODE_TLS,
    Country::TGO => COUNTRY_CODE_TGO,
    Country::TKL => COUNTRY_CODE_TKL,
    Country::TON => COUNTRY_CODE_TON,
    Country::TTO => COUNTRY_CODE_TTO,
    Country::TUN => COUNTRY_CODE_TUN,
    Country::TUR => COUNTRY_CODE_TUR,
    Country::TKM => COUNTRY_CODE_TKM,
    Country::TCA => COUNTRY_CODE_TCA,
    Country::TUV => COUNTRY_CODE_TUV,
    Country::UGA => COUNTRY_CODE_UGA,
    Country::UKR => COUNTRY_CODE_UKR,
    Country::ARE => COUNTRY_CODE_ARE,
    Country::GBR => COUNTRY_CODE_GBR,
    Country::USA => COUNTRY_CODE_USA,
    Country::UMI => COUNTRY_CODE_UMI,
    Country::URY => COUNTRY_CODE_URY,
    Country::UZB => COUNTRY_CODE_UZB,
    Country::VUT => COUNTRY_CODE_VUT,
    Country::VEN => COUNTRY_CODE_VEN,
    Country::VNM => COUNTRY_CODE_VNM,
    Country::VGB => COUNTRY_CODE_VGB,
    Country::VIR => COUNTRY_CODE_VIR,
    Country::WLF => COUNTRY_CODE_WLF,
    Country::ESH => COUNTRY_CODE_ESH,
    Country::YEM => COUNTRY_CODE_YEM,
    Country::ZMB => COUNTRY_CODE_ZMB,
    Country::ZWE => COUNTRY_CODE_ZWE,
}
}

/// Function to convert from country_code to Country
pub fn country_from_country_code(country_code: usize) -> Option<Country>
{
match country_code {
    COUNTRY_CODE_AFG => Some(Country::AFG),
    COUNTRY_CODE_ALA => Some(Country::ALA),
    COUNTRY_CODE_ALB => Some(Country::ALB),
    COUNTRY_CODE_DZA => Some(Country::DZA),
    COUNTRY_CODE_ASM => Some(Country::ASM),
    COUNTRY_CODE_AND => Some(Country::AND),
    COUNTRY_CODE_AGO => Some(Country::AGO),
    COUNTRY_CODE_AIA => Some(Country::AIA),
    COUNTRY_CODE_ATA => Some(Country::ATA),
    COUNTRY_CODE_ATG => Some(Country::ATG),
    COUNTRY_CODE_ARG => Some(Country::ARG),
    COUNTRY_CODE_ARM => Some(Country::ARM),
    COUNTRY_CODE_ABW => Some(Country::ABW),
    COUNTRY_CODE_AUS => Some(Country::AUS),
    COUNTRY_CODE_AUT => Some(Country::AUT),
    COUNTRY_CODE_AZE => Some(Country::AZE),
    COUNTRY_CODE_BHS => Some(Country::BHS),
    COUNTRY_CODE_BHR => Some(Country::BHR),
    COUNTRY_CODE_BGD => Some(Country::BGD),
    COUNTRY_CODE_BRB => Some(Country::BRB),
    COUNTRY_CODE_BLR => Some(Country::BLR),
    COUNTRY_CODE_BEL => Some(Country::BEL),
    COUNTRY_CODE_BLZ => Some(Country::BLZ),
    COUNTRY_CODE_BEN => Some(Country::BEN),
    COUNTRY_CODE_BMU => Some(Country::BMU),
    COUNTRY_CODE_BTN => Some(Country::BTN),
    COUNTRY_CODE_BOL => Some(Country::BOL),
    COUNTRY_CODE_BES => Some(Country::BES),
    COUNTRY_CODE_BIH => Some(Country::BIH),
    COUNTRY_CODE_BWA => Some(Country::BWA),
    COUNTRY_CODE_BVT => Some(Country::BVT),
    COUNTRY_CODE_BRA => Some(Country::BRA),
    COUNTRY_CODE_IOT => Some(Country::IOT),
    COUNTRY_CODE_BRN => Some(Country::BRN),
    COUNTRY_CODE_BGR => Some(Country::BGR),
    COUNTRY_CODE_BFA => Some(Country::BFA),
    COUNTRY_CODE_BDI => Some(Country::BDI),
    COUNTRY_CODE_CPV => Some(Country::CPV),
    COUNTRY_CODE_KHM => Some(Country::KHM),
    COUNTRY_CODE_CMR => Some(Country::CMR),
    COUNTRY_CODE_CAN => Some(Country::CAN),
    COUNTRY_CODE_CYM => Some(Country::CYM),
    COUNTRY_CODE_CAF => Some(Country::CAF),
    COUNTRY_CODE_TCD => Some(Country::TCD),
    COUNTRY_CODE_CHL => Some(Country::CHL),
    COUNTRY_CODE_CHN => Some(Country::CHN),
    COUNTRY_CODE_CXR => Some(Country::CXR),
    COUNTRY_CODE_CCK => Some(Country::CCK),
    COUNTRY_CODE_COL => Some(Country::COL),
    COUNTRY_CODE_COM => Some(Country::COM),
    COUNTRY_CODE_COG => Some(Country::COG),
    COUNTRY_CODE_COD => Some(Country::COD),
    COUNTRY_CODE_COK => Some(Country::COK),
    COUNTRY_CODE_CRI => Some(Country::CRI),
    COUNTRY_CODE_CIV => Some(Country::CIV),
    COUNTRY_CODE_HRV => Some(Country::HRV),
    COUNTRY_CODE_CUB => Some(Country::CUB),
    COUNTRY_CODE_CUW => Some(Country::CUW),
    COUNTRY_CODE_CYP => Some(Country::CYP),
    COUNTRY_CODE_CZE => Some(Country::CZE),
    COUNTRY_CODE_DNK => Some(Country::DNK),
    COUNTRY_CODE_DJI => Some(Country::DJI),
    COUNTRY_CODE_DMA => Some(Country::DMA),
    COUNTRY_CODE_DOM => Some(Country::DOM),
    COUNTRY_CODE_ECU => Some(Country::ECU),
    COUNTRY_CODE_EGY => Some(Country::EGY),
    COUNTRY_CODE_SLV => Some(Country::SLV),
    COUNTRY_CODE_GNQ => Some(Country::GNQ),
    COUNTRY_CODE_ERI => Some(Country::ERI),
    COUNTRY_CODE_EST => Some(Country::EST),
    COUNTRY_CODE_SWZ => Some(Country::SWZ),
    COUNTRY_CODE_ETH => Some(Country::ETH),
    COUNTRY_CODE_FLK => Some(Country::FLK),
    COUNTRY_CODE_FRO => Some(Country::FRO),
    COUNTRY_CODE_FJI => Some(Country::FJI),
    COUNTRY_CODE_FIN => Some(Country::FIN),
    COUNTRY_CODE_FRA => Some(Country::FRA),
    COUNTRY_CODE_GUF => Some(Country::GUF),
    COUNTRY_CODE_PYF => Some(Country::PYF),
    COUNTRY_CODE_ATF => Some(Country::ATF),
    COUNTRY_CODE_GAB => Some(Country::GAB),
    COUNTRY_CODE_GMB => Some(Country::GMB),
    COUNTRY_CODE_GEO => Some(Country::GEO),
    COUNTRY_CODE_DEU => Some(Country::DEU),
    COUNTRY_CODE_GHA => Some(Country::GHA),
    COUNTRY_CODE_GIB => Some(Country::GIB),
    COUNTRY_CODE_GRC => Some(Country::GRC),
    COUNTRY_CODE_GRL => Some(Country::GRL),
    COUNTRY_CODE_GRD => Some(Country::GRD),
    COUNTRY_CODE_GLP => Some(Country::GLP),
    COUNTRY_CODE_GUM => Some(Country::GUM),
    COUNTRY_CODE_GTM => Some(Country::GTM),
    COUNTRY_CODE_GGY => Some(Country::GGY),
    COUNTRY_CODE_GIN => Some(Country::GIN),
    COUNTRY_CODE_GNB => Some(Country::GNB),
    COUNTRY_CODE_GUY => Some(Country::GUY),
    COUNTRY_CODE_HTI => Some(Country::HTI),
    COUNTRY_CODE_HMD => Some(Country::HMD),
    COUNTRY_CODE_VAT => Some(Country::VAT),
    COUNTRY_CODE_HND => Some(Country::HND),
    COUNTRY_CODE_HKG => Some(Country::HKG),
    COUNTRY_CODE_HUN => Some(Country::HUN),
    COUNTRY_CODE_ISL => Some(Country::ISL),
    COUNTRY_CODE_IND => Some(Country::IND),
    COUNTRY_CODE_IDN => Some(Country::IDN),
    COUNTRY_CODE_IRN => Some(Country::IRN),
    COUNTRY_CODE_IRQ => Some(Country::IRQ),
    COUNTRY_CODE_IRL => Some(Country::IRL),
    COUNTRY_CODE_IMN => Some(Country::IMN),
    COUNTRY_CODE_ISR => Some(Country::ISR),
    COUNTRY_CODE_ITA => Some(Country::ITA),
    COUNTRY_CODE_JAM => Some(Country::JAM),
    COUNTRY_CODE_JPN => Some(Country::JPN),
    COUNTRY_CODE_JEY => Some(Country::JEY),
    COUNTRY_CODE_JOR => Some(Country::JOR),
    COUNTRY_CODE_KAZ => Some(Country::KAZ),
    COUNTRY_CODE_KEN => Some(Country::KEN),
    COUNTRY_CODE_KIR => Some(Country::KIR),
    COUNTRY_CODE_PRK => Some(Country::PRK),
    COUNTRY_CODE_KOR => Some(Country::KOR),
    COUNTRY_CODE_KWT => Some(Country::KWT),
    COUNTRY_CODE_KGZ => Some(Country::KGZ),
    COUNTRY_CODE_LAO => Some(Country::LAO),
    COUNTRY_CODE_LVA => Some(Country::LVA),
    COUNTRY_CODE_LBN => Some(Country::LBN),
    COUNTRY_CODE_LSO => Some(Country::LSO),
    COUNTRY_CODE_LBR => Some(Country::LBR),
    COUNTRY_CODE_LBY => Some(Country::LBY),
    COUNTRY_CODE_LIE => Some(Country::LIE),
    COUNTRY_CODE_LTU => Some(Country::LTU),
    COUNTRY_CODE_LUX => Some(Country::LUX),
    COUNTRY_CODE_MAC => Some(Country::MAC),
    COUNTRY_CODE_MDG => Some(Country::MDG),
    COUNTRY_CODE_MWI => Some(Country::MWI),
    COUNTRY_CODE_MYS => Some(Country::MYS),
    COUNTRY_CODE_MDV => Some(Country::MDV),
    COUNTRY_CODE_MLI => Some(Country::MLI),
    COUNTRY_CODE_MLT => Some(Country::MLT),
    COUNTRY_CODE_MHL => Some(Country::MHL),
    COUNTRY_CODE_MTQ => Some(Country::MTQ),
    COUNTRY_CODE_MRT => Some(Country::MRT),
    COUNTRY_CODE_MUS => Some(Country::MUS),
    COUNTRY_CODE_MYT => Some(Country::MYT),
    COUNTRY_CODE_MEX => Some(Country::MEX),
    COUNTRY_CODE_FSM => Some(Country::FSM),
    COUNTRY_CODE_MDA => Some(Country::MDA),
    COUNTRY_CODE_MCO => Some(Country::MCO),
    COUNTRY_CODE_MNG => Some(Country::MNG),
    COUNTRY_CODE_MNE => Some(Country::MNE),
    COUNTRY_CODE_MSR => Some(Country::MSR),
    COUNTRY_CODE_MAR => Some(Country::MAR),
    COUNTRY_CODE_MOZ => Some(Country::MOZ),
    COUNTRY_CODE_MMR => Some(Country::MMR),
    COUNTRY_CODE_NAM => Some(Country::NAM),
    COUNTRY_CODE_NRU => Some(Country::NRU),
    COUNTRY_CODE_NPL => Some(Country::NPL),
    COUNTRY_CODE_NLD => Some(Country::NLD),
    COUNTRY_CODE_NCL => Some(Country::NCL),
    COUNTRY_CODE_NZL => Some(Country::NZL),
    COUNTRY_CODE_NIC => Some(Country::NIC),
    COUNTRY_CODE_NER => Some(Country::NER),
    COUNTRY_CODE_NGA => Some(Country::NGA),
    COUNTRY_CODE_NIU => Some(Country::NIU),
    COUNTRY_CODE_NFK => Some(Country::NFK),
    COUNTRY_CODE_MKD => Some(Country::MKD),
    COUNTRY_CODE_MNP => Some(Country::MNP),
    COUNTRY_CODE_NOR => Some(Country::NOR),
    COUNTRY_CODE_OMN => Some(Country::OMN),
    COUNTRY_CODE_PAK => Some(Country::PAK),
    COUNTRY_CODE_PLW => Some(Country::PLW),
    COUNTRY_CODE_PSE => Some(Country::PSE),
    COUNTRY_CODE_PAN => Some(Country::PAN),
    COUNTRY_CODE_PNG => Some(Country::PNG),
    COUNTRY_CODE_PRY => Some(Country::PRY),
    COUNTRY_CODE_PER => Some(Country::PER),
    COUNTRY_CODE_PHL => Some(Country::PHL),
    COUNTRY_CODE_PCN => Some(Country::PCN),
    COUNTRY_CODE_POL => Some(Country::POL),
    COUNTRY_CODE_PRT => Some(Country::PRT),
    COUNTRY_CODE_PRI => Some(Country::PRI),
    COUNTRY_CODE_QAT => Some(Country::QAT),
    COUNTRY_CODE_REU => Some(Country::REU),
    COUNTRY_CODE_ROU => Some(Country::ROU),
    COUNTRY_CODE_RUS => Some(Country::RUS),
    COUNTRY_CODE_RWA => Some(Country::RWA),
    COUNTRY_CODE_BLM => Some(Country::BLM),
    COUNTRY_CODE_SHN => Some(Country::SHN),
    COUNTRY_CODE_KNA => Some(Country::KNA),
    COUNTRY_CODE_LCA => Some(Country::LCA),
    COUNTRY_CODE_MAF => Some(Country::MAF),
    COUNTRY_CODE_SPM => Some(Country::SPM),
    COUNTRY_CODE_VCT => Some(Country::VCT),
    COUNTRY_CODE_WSM => Some(Country::WSM),
    COUNTRY_CODE_SMR => Some(Country::SMR),
    COUNTRY_CODE_STP => Some(Country::STP),
    COUNTRY_CODE_SAU => Some(Country::SAU),
    COUNTRY_CODE_SEN => Some(Country::SEN),
    COUNTRY_CODE_SRB => Some(Country::SRB),
    COUNTRY_CODE_SYC => Some(Country::SYC),
    COUNTRY_CODE_SLE => Some(Country::SLE),
    COUNTRY_CODE_SGP => Some(Country::SGP),
    COUNTRY_CODE_SXM => Some(Country::SXM),
    COUNTRY_CODE_SVK => Some(Country::SVK),
    COUNTRY_CODE_SVN => Some(Country::SVN),
    COUNTRY_CODE_SLB => Some(Country::SLB),
    COUNTRY_CODE_SOM => Some(Country::SOM),
    COUNTRY_CODE_ZAF => Some(Country::ZAF),
    COUNTRY_CODE_SGS => Some(Country::SGS),
    COUNTRY_CODE_SSD => Some(Country::SSD),
    COUNTRY_CODE_ESP => Some(Country::ESP),
    COUNTRY_CODE_LKA => Some(Country::LKA),
    COUNTRY_CODE_SDN => Some(Country::SDN),
    COUNTRY_CODE_SUR => Some(Country::SUR),
    COUNTRY_CODE_SJM => Some(Country::SJM),
    COUNTRY_CODE_SWE => Some(Country::SWE),
    COUNTRY_CODE_CHE => Some(Country::CHE),
    COUNTRY_CODE_SYR => Some(Country::SYR),
    COUNTRY_CODE_TWN => Some(Country::TWN),
    COUNTRY_CODE_TJK => Some(Country::TJK),
    COUNTRY_CODE_TZA => Some(Country::TZA),
    COUNTRY_CODE_THA => Some(Country::THA),
    COUNTRY_CODE_TLS => Some(Country::TLS),
    COUNTRY_CODE_TGO => Some(Country::TGO),
    COUNTRY_CODE_TKL => Some(Country::TKL),
    COUNTRY_CODE_TON => Some(Country::TON),
    COUNTRY_CODE_TTO => Some(Country::TTO),
    COUNTRY_CODE_TUN => Some(Country::TUN),
    COUNTRY_CODE_TUR => Some(Country::TUR),
    COUNTRY_CODE_TKM => Some(Country::TKM),
    COUNTRY_CODE_TCA => Some(Country::TCA),
    COUNTRY_CODE_TUV => Some(Country::TUV),
    COUNTRY_CODE_UGA => Some(Country::UGA),
    COUNTRY_CODE_UKR => Some(Country::UKR),
    COUNTRY_CODE_ARE => Some(Country::ARE),
    COUNTRY_CODE_GBR => Some(Country::GBR),
    COUNTRY_CODE_USA => Some(Country::USA),
    COUNTRY_CODE_UMI => Some(Country::UMI),
    COUNTRY_CODE_URY => Some(Country::URY),
    COUNTRY_CODE_UZB => Some(Country::UZB),
    COUNTRY_CODE_VUT => Some(Country::VUT),
    COUNTRY_CODE_VEN => Some(Country::VEN),
    COUNTRY_CODE_VNM => Some(Country::VNM),
    COUNTRY_CODE_VGB => Some(Country::VGB),
    COUNTRY_CODE_VIR => Some(Country::VIR),
    COUNTRY_CODE_WLF => Some(Country::WLF),
    COUNTRY_CODE_ESH => Some(Country::ESH),
    COUNTRY_CODE_YEM => Some(Country::YEM),
    COUNTRY_CODE_ZMB => Some(Country::ZMB),
    COUNTRY_CODE_ZWE => Some(Country::ZWE),
    _ => Option::None,
}
}
/// Constants for `name`
const NAME_AFG: &'static str = "Afghanistan";
const NAME_ALA: &'static str = "Ãland Islands";
const NAME_ALB: &'static str = "Albania";
const NAME_DZA: &'static str = "Algeria";
const NAME_ASM: &'static str = "American Samoa";
const NAME_AND: &'static str = "Andorra";
const NAME_AGO: &'static str = "Angola";
const NAME_AIA: &'static str = "Anguilla";
const NAME_ATA: &'static str = "Antarctica";
const NAME_ATG: &'static str = "Antigua and Barbuda";
const NAME_ARG: &'static str = "Argentina";
const NAME_ARM: &'static str = "Armenia";
const NAME_ABW: &'static str = "Aruba";
const NAME_AUS: &'static str = "Australia";
const NAME_AUT: &'static str = "Austria";
const NAME_AZE: &'static str = "Azerbaijan";
const NAME_BHS: &'static str = "Bahamas";
const NAME_BHR: &'static str = "Bahrain";
const NAME_BGD: &'static str = "Bangladesh";
const NAME_BRB: &'static str = "Barbados";
const NAME_BLR: &'static str = "Belarus";
const NAME_BEL: &'static str = "Belgium";
const NAME_BLZ: &'static str = "Belize";
const NAME_BEN: &'static str = "Benin";
const NAME_BMU: &'static str = "Bermuda";
const NAME_BTN: &'static str = "Bhutan";
const NAME_BOL: &'static str = "Bolivia (Plurinational State of)";
const NAME_BES: &'static str = "Bonaire, Sint Eustatius and Saba";
const NAME_BIH: &'static str = "Bosnia and Herzegovina";
const NAME_BWA: &'static str = "Botswana";
const NAME_BVT: &'static str = "Bouvet Island";
const NAME_BRA: &'static str = "Brazil";
const NAME_IOT: &'static str = "British Indian Ocean Territory";
const NAME_BRN: &'static str = "Brunei Darussalam";
const NAME_BGR: &'static str = "Bulgaria";
const NAME_BFA: &'static str = "Burkina Faso";
const NAME_BDI: &'static str = "Burundi";
const NAME_CPV: &'static str = "Cabo Verde";
const NAME_KHM: &'static str = "Cambodia";
const NAME_CMR: &'static str = "Cameroon";
const NAME_CAN: &'static str = "Canada";
const NAME_CYM: &'static str = "Cayman Islands";
const NAME_CAF: &'static str = "Central African Republic";
const NAME_TCD: &'static str = "Chad";
const NAME_CHL: &'static str = "Chile";
const NAME_CHN: &'static str = "China";
const NAME_CXR: &'static str = "Christmas Island";
const NAME_CCK: &'static str = "Cocos (Keeling) Islands";
const NAME_COL: &'static str = "Colombia";
const NAME_COM: &'static str = "Comoros";
const NAME_COG: &'static str = "Congo";
const NAME_COD: &'static str = "Congo, Democratic Republic of the";
const NAME_COK: &'static str = "Cook Islands";
const NAME_CRI: &'static str = "Costa Rica";
const NAME_CIV: &'static str = "CÃ´te d'Ivoire";
const NAME_HRV: &'static str = "Croatia";
const NAME_CUB: &'static str = "Cuba";
const NAME_CUW: &'static str = "CuraÃ§ao";
const NAME_CYP: &'static str = "Cyprus";
const NAME_CZE: &'static str = "Czechia";
const NAME_DNK: &'static str = "Denmark";
const NAME_DJI: &'static str = "Djibouti";
const NAME_DMA: &'static str = "Dominica";
const NAME_DOM: &'static str = "Dominican Republic";
const NAME_ECU: &'static str = "Ecuador";
const NAME_EGY: &'static str = "Egypt";
const NAME_SLV: &'static str = "El Salvador";
const NAME_GNQ: &'static str = "Equatorial Guinea";
const NAME_ERI: &'static str = "Eritrea";
const NAME_EST: &'static str = "Estonia";
const NAME_SWZ: &'static str = "Eswatini";
const NAME_ETH: &'static str = "Ethiopia";
const NAME_FLK: &'static str = "Falkland Islands (Malvinas)";
const NAME_FRO: &'static str = "Faroe Islands";
const NAME_FJI: &'static str = "Fiji";
const NAME_FIN: &'static str = "Finland";
const NAME_FRA: &'static str = "France";
const NAME_GUF: &'static str = "French Guiana";
const NAME_PYF: &'static str = "French Polynesia";
const NAME_ATF: &'static str = "French Southern Territories";
const NAME_GAB: &'static str = "Gabon";
const NAME_GMB: &'static str = "Gambia";
const NAME_GEO: &'static str = "Georgia";
const NAME_DEU: &'static str = "Germany";
const NAME_GHA: &'static str = "Ghana";
const NAME_GIB: &'static str = "Gibraltar";
const NAME_GRC: &'static str = "Greece";
const NAME_GRL: &'static str = "Greenland";
const NAME_GRD: &'static str = "Grenada";
const NAME_GLP: &'static str = "Guadeloupe";
const NAME_GUM: &'static str = "Guam";
const NAME_GTM: &'static str = "Guatemala";
const NAME_GGY: &'static str = "Guernsey";
const NAME_GIN: &'static str = "Guinea";
const NAME_GNB: &'static str = "Guinea-Bissau";
const NAME_GUY: &'static str = "Guyana";
const NAME_HTI: &'static str = "Haiti";
const NAME_HMD: &'static str = "Heard Island and McDonald Islands";
const NAME_VAT: &'static str = "Holy See";
const NAME_HND: &'static str = "Honduras";
const NAME_HKG: &'static str = "Hong Kong";
const NAME_HUN: &'static str = "Hungary";
const NAME_ISL: &'static str = "Iceland";
const NAME_IND: &'static str = "India";
const NAME_IDN: &'static str = "Indonesia";
const NAME_IRN: &'static str = "Iran (Islamic Republic of)";
const NAME_IRQ: &'static str = "Iraq";
const NAME_IRL: &'static str = "Ireland";
const NAME_IMN: &'static str = "Isle of Man";
const NAME_ISR: &'static str = "Israel";
const NAME_ITA: &'static str = "Italy";
const NAME_JAM: &'static str = "Jamaica";
const NAME_JPN: &'static str = "Japan";
const NAME_JEY: &'static str = "Jersey";
const NAME_JOR: &'static str = "Jordan";
const NAME_KAZ: &'static str = "Kazakhstan";
const NAME_KEN: &'static str = "Kenya";
const NAME_KIR: &'static str = "Kiribati";
const NAME_PRK: &'static str = "Korea (Democratic People's Republic of)";
const NAME_KOR: &'static str = "Korea, Republic of";
const NAME_KWT: &'static str = "Kuwait";
const NAME_KGZ: &'static str = "Kyrgyzstan";
const NAME_LAO: &'static str = "Lao People's Democratic Republic";
const NAME_LVA: &'static str = "Latvia";
const NAME_LBN: &'static str = "Lebanon";
const NAME_LSO: &'static str = "Lesotho";
const NAME_LBR: &'static str = "Liberia";
const NAME_LBY: &'static str = "Libya";
const NAME_LIE: &'static str = "Liechtenstein";
const NAME_LTU: &'static str = "Lithuania";
const NAME_LUX: &'static str = "Luxembourg";
const NAME_MAC: &'static str = "Macao";
const NAME_MDG: &'static str = "Madagascar";
const NAME_MWI: &'static str = "Malawi";
const NAME_MYS: &'static str = "Malaysia";
const NAME_MDV: &'static str = "Maldives";
const NAME_MLI: &'static str = "Mali";
const NAME_MLT: &'static str = "Malta";
const NAME_MHL: &'static str = "Marshall Islands";
const NAME_MTQ: &'static str = "Martinique";
const NAME_MRT: &'static str = "Mauritania";
const NAME_MUS: &'static str = "Mauritius";
const NAME_MYT: &'static str = "Mayotte";
const NAME_MEX: &'static str = "Mexico";
const NAME_FSM: &'static str = "Micronesia (Federated States of)";
const NAME_MDA: &'static str = "Moldova, Republic of";
const NAME_MCO: &'static str = "Monaco";
const NAME_MNG: &'static str = "Mongolia";
const NAME_MNE: &'static str = "Montenegro";
const NAME_MSR: &'static str = "Montserrat";
const NAME_MAR: &'static str = "Morocco";
const NAME_MOZ: &'static str = "Mozambique";
const NAME_MMR: &'static str = "Myanmar";
const NAME_NAM: &'static str = "Namibia";
const NAME_NRU: &'static str = "Nauru";
const NAME_NPL: &'static str = "Nepal";
const NAME_NLD: &'static str = "Netherlands";
const NAME_NCL: &'static str = "New Caledonia";
const NAME_NZL: &'static str = "New Zealand";
const NAME_NIC: &'static str = "Nicaragua";
const NAME_NER: &'static str = "Niger";
const NAME_NGA: &'static str = "Nigeria";
const NAME_NIU: &'static str = "Niue";
const NAME_NFK: &'static str = "Norfolk Island";
const NAME_MKD: &'static str = "North Macedonia";
const NAME_MNP: &'static str = "Northern Mariana Islands";
const NAME_NOR: &'static str = "Norway";
const NAME_OMN: &'static str = "Oman";
const NAME_PAK: &'static str = "Pakistan";
const NAME_PLW: &'static str = "Palau";
const NAME_PSE: &'static str = "Palestine, State of";
const NAME_PAN: &'static str = "Panama";
const NAME_PNG: &'static str = "Papua New Guinea";
const NAME_PRY: &'static str = "Paraguay";
const NAME_PER: &'static str = "Peru";
const NAME_PHL: &'static str = "Philippines";
const NAME_PCN: &'static str = "Pitcairn";
const NAME_POL: &'static str = "Poland";
const NAME_PRT: &'static str = "Portugal";
const NAME_PRI: &'static str = "Puerto Rico";
const NAME_QAT: &'static str = "Qatar";
const NAME_REU: &'static str = "RÃ©union";
const NAME_ROU: &'static str = "Romania";
const NAME_RUS: &'static str = "Russian Federation";
const NAME_RWA: &'static str = "Rwanda";
const NAME_BLM: &'static str = "Saint BarthÃ©lemy";
const NAME_SHN: &'static str = "Saint Helena, Ascension and Tristan da Cunha";
const NAME_KNA: &'static str = "Saint Kitts and Nevis";
const NAME_LCA: &'static str = "Saint Lucia";
const NAME_MAF: &'static str = "Saint Martin (French part)";
const NAME_SPM: &'static str = "Saint Pierre and Miquelon";
const NAME_VCT: &'static str = "Saint Vincent and the Grenadines";
const NAME_WSM: &'static str = "Samoa";
const NAME_SMR: &'static str = "San Marino";
const NAME_STP: &'static str = "Sao Tome and Principe";
const NAME_SAU: &'static str = "Saudi Arabia";
const NAME_SEN: &'static str = "Senegal";
const NAME_SRB: &'static str = "Serbia";
const NAME_SYC: &'static str = "Seychelles";
const NAME_SLE: &'static str = "Sierra Leone";
const NAME_SGP: &'static str = "Singapore";
const NAME_SXM: &'static str = "Sint Maarten (Dutch part)";
const NAME_SVK: &'static str = "Slovakia";
const NAME_SVN: &'static str = "Slovenia";
const NAME_SLB: &'static str = "Solomon Islands";
const NAME_SOM: &'static str = "Somalia";
const NAME_ZAF: &'static str = "South Africa";
const NAME_SGS: &'static str = "South Georgia and the South Sandwich Islands";
const NAME_SSD: &'static str = "South Sudan";
const NAME_ESP: &'static str = "Spain";
const NAME_LKA: &'static str = "Sri Lanka";
const NAME_SDN: &'static str = "Sudan";
const NAME_SUR: &'static str = "Suriname";
const NAME_SJM: &'static str = "Svalbard and Jan Mayen";
const NAME_SWE: &'static str = "Sweden";
const NAME_CHE: &'static str = "Switzerland";
const NAME_SYR: &'static str = "Syrian Arab Republic";
const NAME_TWN: &'static str = "Taiwan, Province of China";
const NAME_TJK: &'static str = "Tajikistan";
const NAME_TZA: &'static str = "Tanzania, United Republic of";
const NAME_THA: &'static str = "Thailand";
const NAME_TLS: &'static str = "Timor-Leste";
const NAME_TGO: &'static str = "Togo";
const NAME_TKL: &'static str = "Tokelau";
const NAME_TON: &'static str = "Tonga";
const NAME_TTO: &'static str = "Trinidad and Tobago";
const NAME_TUN: &'static str = "Tunisia";
const NAME_TUR: &'static str = "Turkey";
const NAME_TKM: &'static str = "Turkmenistan";
const NAME_TCA: &'static str = "Turks and Caicos Islands";
const NAME_TUV: &'static str = "Tuvalu";
const NAME_UGA: &'static str = "Uganda";
const NAME_UKR: &'static str = "Ukraine";
const NAME_ARE: &'static str = "United Arab Emirates";
const NAME_GBR: &'static str = "United Kingdom of Great Britain and Northern Ireland";
const NAME_USA: &'static str = "United States of America";
const NAME_UMI: &'static str = "United States Minor Outlying Islands";
const NAME_URY: &'static str = "Uruguay";
const NAME_UZB: &'static str = "Uzbekistan";
const NAME_VUT: &'static str = "Vanuatu";
const NAME_VEN: &'static str = "Venezuela (Bolivarian Republic of)";
const NAME_VNM: &'static str = "Viet Nam";
const NAME_VGB: &'static str = "Virgin Islands (British)";
const NAME_VIR: &'static str = "Virgin Islands (U.S.)";
const NAME_WLF: &'static str = "Wallis and Futuna";
const NAME_ESH: &'static str = "Western Sahara";
const NAME_YEM: &'static str = "Yemen";
const NAME_ZMB: &'static str = "Zambia";
const NAME_ZWE: &'static str = "Zimbabwe";

/// Function to convert from Country to name
pub const fn country_as_name(country: &Country) -> &'static str
{
match country {
    Country::AFG => NAME_AFG,
    Country::ALA => NAME_ALA,
    Country::ALB => NAME_ALB,
    Country::DZA => NAME_DZA,
    Country::ASM => NAME_ASM,
    Country::AND => NAME_AND,
    Country::AGO => NAME_AGO,
    Country::AIA => NAME_AIA,
    Country::ATA => NAME_ATA,
    Country::ATG => NAME_ATG,
    Country::ARG => NAME_ARG,
    Country::ARM => NAME_ARM,
    Country::ABW => NAME_ABW,
    Country::AUS => NAME_AUS,
    Country::AUT => NAME_AUT,
    Country::AZE => NAME_AZE,
    Country::BHS => NAME_BHS,
    Country::BHR => NAME_BHR,
    Country::BGD => NAME_BGD,
    Country::BRB => NAME_BRB,
    Country::BLR => NAME_BLR,
    Country::BEL => NAME_BEL,
    Country::BLZ => NAME_BLZ,
    Country::BEN => NAME_BEN,
    Country::BMU => NAME_BMU,
    Country::BTN => NAME_BTN,
    Country::BOL => NAME_BOL,
    Country::BES => NAME_BES,
    Country::BIH => NAME_BIH,
    Country::BWA => NAME_BWA,
    Country::BVT => NAME_BVT,
    Country::BRA => NAME_BRA,
    Country::IOT => NAME_IOT,
    Country::BRN => NAME_BRN,
    Country::BGR => NAME_BGR,
    Country::BFA => NAME_BFA,
    Country::BDI => NAME_BDI,
    Country::CPV => NAME_CPV,
    Country::KHM => NAME_KHM,
    Country::CMR => NAME_CMR,
    Country::CAN => NAME_CAN,
    Country::CYM => NAME_CYM,
    Country::CAF => NAME_CAF,
    Country::TCD => NAME_TCD,
    Country::CHL => NAME_CHL,
    Country::CHN => NAME_CHN,
    Country::CXR => NAME_CXR,
    Country::CCK => NAME_CCK,
    Country::COL => NAME_COL,
    Country::COM => NAME_COM,
    Country::COG => NAME_COG,
    Country::COD => NAME_COD,
    Country::COK => NAME_COK,
    Country::CRI => NAME_CRI,
    Country::CIV => NAME_CIV,
    Country::HRV => NAME_HRV,
    Country::CUB => NAME_CUB,
    Country::CUW => NAME_CUW,
    Country::CYP => NAME_CYP,
    Country::CZE => NAME_CZE,
    Country::DNK => NAME_DNK,
    Country::DJI => NAME_DJI,
    Country::DMA => NAME_DMA,
    Country::DOM => NAME_DOM,
    Country::ECU => NAME_ECU,
    Country::EGY => NAME_EGY,
    Country::SLV => NAME_SLV,
    Country::GNQ => NAME_GNQ,
    Country::ERI => NAME_ERI,
    Country::EST => NAME_EST,
    Country::SWZ => NAME_SWZ,
    Country::ETH => NAME_ETH,
    Country::FLK => NAME_FLK,
    Country::FRO => NAME_FRO,
    Country::FJI => NAME_FJI,
    Country::FIN => NAME_FIN,
    Country::FRA => NAME_FRA,
    Country::GUF => NAME_GUF,
    Country::PYF => NAME_PYF,
    Country::ATF => NAME_ATF,
    Country::GAB => NAME_GAB,
    Country::GMB => NAME_GMB,
    Country::GEO => NAME_GEO,
    Country::DEU => NAME_DEU,
    Country::GHA => NAME_GHA,
    Country::GIB => NAME_GIB,
    Country::GRC => NAME_GRC,
    Country::GRL => NAME_GRL,
    Country::GRD => NAME_GRD,
    Country::GLP => NAME_GLP,
    Country::GUM => NAME_GUM,
    Country::GTM => NAME_GTM,
    Country::GGY => NAME_GGY,
    Country::GIN => NAME_GIN,
    Country::GNB => NAME_GNB,
    Country::GUY => NAME_GUY,
    Country::HTI => NAME_HTI,
    Country::HMD => NAME_HMD,
    Country::VAT => NAME_VAT,
    Country::HND => NAME_HND,
    Country::HKG => NAME_HKG,
    Country::HUN => NAME_HUN,
    Country::ISL => NAME_ISL,
    Country::IND => NAME_IND,
    Country::IDN => NAME_IDN,
    Country::IRN => NAME_IRN,
    Country::IRQ => NAME_IRQ,
    Country::IRL => NAME_IRL,
    Country::IMN => NAME_IMN,
    Country::ISR => NAME_ISR,
    Country::ITA => NAME_ITA,
    Country::JAM => NAME_JAM,
    Country::JPN => NAME_JPN,
    Country::JEY => NAME_JEY,
    Country::JOR => NAME_JOR,
    Country::KAZ => NAME_KAZ,
    Country::KEN => NAME_KEN,
    Country::KIR => NAME_KIR,
    Country::PRK => NAME_PRK,
    Country::KOR => NAME_KOR,
    Country::KWT => NAME_KWT,
    Country::KGZ => NAME_KGZ,
    Country::LAO => NAME_LAO,
    Country::LVA => NAME_LVA,
    Country::LBN => NAME_LBN,
    Country::LSO => NAME_LSO,
    Country::LBR => NAME_LBR,
    Country::LBY => NAME_LBY,
    Country::LIE => NAME_LIE,
    Country::LTU => NAME_LTU,
    Country::LUX => NAME_LUX,
    Country::MAC => NAME_MAC,
    Country::MDG => NAME_MDG,
    Country::MWI => NAME_MWI,
    Country::MYS => NAME_MYS,
    Country::MDV => NAME_MDV,
    Country::MLI => NAME_MLI,
    Country::MLT => NAME_MLT,
    Country::MHL => NAME_MHL,
    Country::MTQ => NAME_MTQ,
    Country::MRT => NAME_MRT,
    Country::MUS => NAME_MUS,
    Country::MYT => NAME_MYT,
    Country::MEX => NAME_MEX,
    Country::FSM => NAME_FSM,
    Country::MDA => NAME_MDA,
    Country::MCO => NAME_MCO,
    Country::MNG => NAME_MNG,
    Country::MNE => NAME_MNE,
    Country::MSR => NAME_MSR,
    Country::MAR => NAME_MAR,
    Country::MOZ => NAME_MOZ,
    Country::MMR => NAME_MMR,
    Country::NAM => NAME_NAM,
    Country::NRU => NAME_NRU,
    Country::NPL => NAME_NPL,
    Country::NLD => NAME_NLD,
    Country::NCL => NAME_NCL,
    Country::NZL => NAME_NZL,
    Country::NIC => NAME_NIC,
    Country::NER => NAME_NER,
    Country::NGA => NAME_NGA,
    Country::NIU => NAME_NIU,
    Country::NFK => NAME_NFK,
    Country::MKD => NAME_MKD,
    Country::MNP => NAME_MNP,
    Country::NOR => NAME_NOR,
    Country::OMN => NAME_OMN,
    Country::PAK => NAME_PAK,
    Country::PLW => NAME_PLW,
    Country::PSE => NAME_PSE,
    Country::PAN => NAME_PAN,
    Country::PNG => NAME_PNG,
    Country::PRY => NAME_PRY,
    Country::PER => NAME_PER,
    Country::PHL => NAME_PHL,
    Country::PCN => NAME_PCN,
    Country::POL => NAME_POL,
    Country::PRT => NAME_PRT,
    Country::PRI => NAME_PRI,
    Country::QAT => NAME_QAT,
    Country::REU => NAME_REU,
    Country::ROU => NAME_ROU,
    Country::RUS => NAME_RUS,
    Country::RWA => NAME_RWA,
    Country::BLM => NAME_BLM,
    Country::SHN => NAME_SHN,
    Country::KNA => NAME_KNA,
    Country::LCA => NAME_LCA,
    Country::MAF => NAME_MAF,
    Country::SPM => NAME_SPM,
    Country::VCT => NAME_VCT,
    Country::WSM => NAME_WSM,
    Country::SMR => NAME_SMR,
    Country::STP => NAME_STP,
    Country::SAU => NAME_SAU,
    Country::SEN => NAME_SEN,
    Country::SRB => NAME_SRB,
    Country::SYC => NAME_SYC,
    Country::SLE => NAME_SLE,
    Country::SGP => NAME_SGP,
    Country::SXM => NAME_SXM,
    Country::SVK => NAME_SVK,
    Country::SVN => NAME_SVN,
    Country::SLB => NAME_SLB,
    Country::SOM => NAME_SOM,
    Country::ZAF => NAME_ZAF,
    Country::SGS => NAME_SGS,
    Country::SSD => NAME_SSD,
    Country::ESP => NAME_ESP,
    Country::LKA => NAME_LKA,
    Country::SDN => NAME_SDN,
    Country::SUR => NAME_SUR,
    Country::SJM => NAME_SJM,
    Country::SWE => NAME_SWE,
    Country::CHE => NAME_CHE,
    Country::SYR => NAME_SYR,
    Country::TWN => NAME_TWN,
    Country::TJK => NAME_TJK,
    Country::TZA => NAME_TZA,
    Country::THA => NAME_THA,
    Country::TLS => NAME_TLS,
    Country::TGO => NAME_TGO,
    Country::TKL => NAME_TKL,
    Country::TON => NAME_TON,
    Country::TTO => NAME_TTO,
    Country::TUN => NAME_TUN,
    Country::TUR => NAME_TUR,
    Country::TKM => NAME_TKM,
    Country::TCA => NAME_TCA,
    Country::TUV => NAME_TUV,
    Country::UGA => NAME_UGA,
    Country::UKR => NAME_UKR,
    Country::ARE => NAME_ARE,
    Country::GBR => NAME_GBR,
    Country::USA => NAME_USA,
    Country::UMI => NAME_UMI,
    Country::URY => NAME_URY,
    Country::UZB => NAME_UZB,
    Country::VUT => NAME_VUT,
    Country::VEN => NAME_VEN,
    Country::VNM => NAME_VNM,
    Country::VGB => NAME_VGB,
    Country::VIR => NAME_VIR,
    Country::WLF => NAME_WLF,
    Country::ESH => NAME_ESH,
    Country::YEM => NAME_YEM,
    Country::ZMB => NAME_ZMB,
    Country::ZWE => NAME_ZWE,
}
}

/// Function to convert from name to Country
pub fn country_from_name(name: &str) -> Option<Country>
{
match name {
    NAME_AFG => Some(Country::AFG),
    NAME_ALA => Some(Country::ALA),
    NAME_ALB => Some(Country::ALB),
    NAME_DZA => Some(Country::DZA),
    NAME_ASM => Some(Country::ASM),
    NAME_AND => Some(Country::AND),
    NAME_AGO => Some(Country::AGO),
    NAME_AIA => Some(Country::AIA),
    NAME_ATA => Some(Country::ATA),
    NAME_ATG => Some(Country::ATG),
    NAME_ARG => Some(Country::ARG),
    NAME_ARM => Some(Country::ARM),
    NAME_ABW => Some(Country::ABW),
    NAME_AUS => Some(Country::AUS),
    NAME_AUT => Some(Country::AUT),
    NAME_AZE => Some(Country::AZE),
    NAME_BHS => Some(Country::BHS),
    NAME_BHR => Some(Country::BHR),
    NAME_BGD => Some(Country::BGD),
    NAME_BRB => Some(Country::BRB),
    NAME_BLR => Some(Country::BLR),
    NAME_BEL => Some(Country::BEL),
    NAME_BLZ => Some(Country::BLZ),
    NAME_BEN => Some(Country::BEN),
    NAME_BMU => Some(Country::BMU),
    NAME_BTN => Some(Country::BTN),
    NAME_BOL => Some(Country::BOL),
    NAME_BES => Some(Country::BES),
    NAME_BIH => Some(Country::BIH),
    NAME_BWA => Some(Country::BWA),
    NAME_BVT => Some(Country::BVT),
    NAME_BRA => Some(Country::BRA),
    NAME_IOT => Some(Country::IOT),
    NAME_BRN => Some(Country::BRN),
    NAME_BGR => Some(Country::BGR),
    NAME_BFA => Some(Country::BFA),
    NAME_BDI => Some(Country::BDI),
    NAME_CPV => Some(Country::CPV),
    NAME_KHM => Some(Country::KHM),
    NAME_CMR => Some(Country::CMR),
    NAME_CAN => Some(Country::CAN),
    NAME_CYM => Some(Country::CYM),
    NAME_CAF => Some(Country::CAF),
    NAME_TCD => Some(Country::TCD),
    NAME_CHL => Some(Country::CHL),
    NAME_CHN => Some(Country::CHN),
    NAME_CXR => Some(Country::CXR),
    NAME_CCK => Some(Country::CCK),
    NAME_COL => Some(Country::COL),
    NAME_COM => Some(Country::COM),
    NAME_COG => Some(Country::COG),
    NAME_COD => Some(Country::COD),
    NAME_COK => Some(Country::COK),
    NAME_CRI => Some(Country::CRI),
    NAME_CIV => Some(Country::CIV),
    NAME_HRV => Some(Country::HRV),
    NAME_CUB => Some(Country::CUB),
    NAME_CUW => Some(Country::CUW),
    NAME_CYP => Some(Country::CYP),
    NAME_CZE => Some(Country::CZE),
    NAME_DNK => Some(Country::DNK),
    NAME_DJI => Some(Country::DJI),
    NAME_DMA => Some(Country::DMA),
    NAME_DOM => Some(Country::DOM),
    NAME_ECU => Some(Country::ECU),
    NAME_EGY => Some(Country::EGY),
    NAME_SLV => Some(Country::SLV),
    NAME_GNQ => Some(Country::GNQ),
    NAME_ERI => Some(Country::ERI),
    NAME_EST => Some(Country::EST),
    NAME_SWZ => Some(Country::SWZ),
    NAME_ETH => Some(Country::ETH),
    NAME_FLK => Some(Country::FLK),
    NAME_FRO => Some(Country::FRO),
    NAME_FJI => Some(Country::FJI),
    NAME_FIN => Some(Country::FIN),
    NAME_FRA => Some(Country::FRA),
    NAME_GUF => Some(Country::GUF),
    NAME_PYF => Some(Country::PYF),
    NAME_ATF => Some(Country::ATF),
    NAME_GAB => Some(Country::GAB),
    NAME_GMB => Some(Country::GMB),
    NAME_GEO => Some(Country::GEO),
    NAME_DEU => Some(Country::DEU),
    NAME_GHA => Some(Country::GHA),
    NAME_GIB => Some(Country::GIB),
    NAME_GRC => Some(Country::GRC),
    NAME_GRL => Some(Country::GRL),
    NAME_GRD => Some(Country::GRD),
    NAME_GLP => Some(Country::GLP),
    NAME_GUM => Some(Country::GUM),
    NAME_GTM => Some(Country::GTM),
    NAME_GGY => Some(Country::GGY),
    NAME_GIN => Some(Country::GIN),
    NAME_GNB => Some(Country::GNB),
    NAME_GUY => Some(Country::GUY),
    NAME_HTI => Some(Country::HTI),
    NAME_HMD => Some(Country::HMD),
    NAME_VAT => Some(Country::VAT),
    NAME_HND => Some(Country::HND),
    NAME_HKG => Some(Country::HKG),
    NAME_HUN => Some(Country::HUN),
    NAME_ISL => Some(Country::ISL),
    NAME_IND => Some(Country::IND),
    NAME_IDN => Some(Country::IDN),
    NAME_IRN => Some(Country::IRN),
    NAME_IRQ => Some(Country::IRQ),
    NAME_IRL => Some(Country::IRL),
    NAME_IMN => Some(Country::IMN),
    NAME_ISR => Some(Country::ISR),
    NAME_ITA => Some(Country::ITA),
    NAME_JAM => Some(Country::JAM),
    NAME_JPN => Some(Country::JPN),
    NAME_JEY => Some(Country::JEY),
    NAME_JOR => Some(Country::JOR),
    NAME_KAZ => Some(Country::KAZ),
    NAME_KEN => Some(Country::KEN),
    NAME_KIR => Some(Country::KIR),
    NAME_PRK => Some(Country::PRK),
    NAME_KOR => Some(Country::KOR),
    NAME_KWT => Some(Country::KWT),
    NAME_KGZ => Some(Country::KGZ),
    NAME_LAO => Some(Country::LAO),
    NAME_LVA => Some(Country::LVA),
    NAME_LBN => Some(Country::LBN),
    NAME_LSO => Some(Country::LSO),
    NAME_LBR => Some(Country::LBR),
    NAME_LBY => Some(Country::LBY),
    NAME_LIE => Some(Country::LIE),
    NAME_LTU => Some(Country::LTU),
    NAME_LUX => Some(Country::LUX),
    NAME_MAC => Some(Country::MAC),
    NAME_MDG => Some(Country::MDG),
    NAME_MWI => Some(Country::MWI),
    NAME_MYS => Some(Country::MYS),
    NAME_MDV => Some(Country::MDV),
    NAME_MLI => Some(Country::MLI),
    NAME_MLT => Some(Country::MLT),
    NAME_MHL => Some(Country::MHL),
    NAME_MTQ => Some(Country::MTQ),
    NAME_MRT => Some(Country::MRT),
    NAME_MUS => Some(Country::MUS),
    NAME_MYT => Some(Country::MYT),
    NAME_MEX => Some(Country::MEX),
    NAME_FSM => Some(Country::FSM),
    NAME_MDA => Some(Country::MDA),
    NAME_MCO => Some(Country::MCO),
    NAME_MNG => Some(Country::MNG),
    NAME_MNE => Some(Country::MNE),
    NAME_MSR => Some(Country::MSR),
    NAME_MAR => Some(Country::MAR),
    NAME_MOZ => Some(Country::MOZ),
    NAME_MMR => Some(Country::MMR),
    NAME_NAM => Some(Country::NAM),
    NAME_NRU => Some(Country::NRU),
    NAME_NPL => Some(Country::NPL),
    NAME_NLD => Some(Country::NLD),
    NAME_NCL => Some(Country::NCL),
    NAME_NZL => Some(Country::NZL),
    NAME_NIC => Some(Country::NIC),
    NAME_NER => Some(Country::NER),
    NAME_NGA => Some(Country::NGA),
    NAME_NIU => Some(Country::NIU),
    NAME_NFK => Some(Country::NFK),
    NAME_MKD => Some(Country::MKD),
    NAME_MNP => Some(Country::MNP),
    NAME_NOR => Some(Country::NOR),
    NAME_OMN => Some(Country::OMN),
    NAME_PAK => Some(Country::PAK),
    NAME_PLW => Some(Country::PLW),
    NAME_PSE => Some(Country::PSE),
    NAME_PAN => Some(Country::PAN),
    NAME_PNG => Some(Country::PNG),
    NAME_PRY => Some(Country::PRY),
    NAME_PER => Some(Country::PER),
    NAME_PHL => Some(Country::PHL),
    NAME_PCN => Some(Country::PCN),
    NAME_POL => Some(Country::POL),
    NAME_PRT => Some(Country::PRT),
    NAME_PRI => Some(Country::PRI),
    NAME_QAT => Some(Country::QAT),
    NAME_REU => Some(Country::REU),
    NAME_ROU => Some(Country::ROU),
    NAME_RUS => Some(Country::RUS),
    NAME_RWA => Some(Country::RWA),
    NAME_BLM => Some(Country::BLM),
    NAME_SHN => Some(Country::SHN),
    NAME_KNA => Some(Country::KNA),
    NAME_LCA => Some(Country::LCA),
    NAME_MAF => Some(Country::MAF),
    NAME_SPM => Some(Country::SPM),
    NAME_VCT => Some(Country::VCT),
    NAME_WSM => Some(Country::WSM),
    NAME_SMR => Some(Country::SMR),
    NAME_STP => Some(Country::STP),
    NAME_SAU => Some(Country::SAU),
    NAME_SEN => Some(Country::SEN),
    NAME_SRB => Some(Country::SRB),
    NAME_SYC => Some(Country::SYC),
    NAME_SLE => Some(Country::SLE),
    NAME_SGP => Some(Country::SGP),
    NAME_SXM => Some(Country::SXM),
    NAME_SVK => Some(Country::SVK),
    NAME_SVN => Some(Country::SVN),
    NAME_SLB => Some(Country::SLB),
    NAME_SOM => Some(Country::SOM),
    NAME_ZAF => Some(Country::ZAF),
    NAME_SGS => Some(Country::SGS),
    NAME_SSD => Some(Country::SSD),
    NAME_ESP => Some(Country::ESP),
    NAME_LKA => Some(Country::LKA),
    NAME_SDN => Some(Country::SDN),
    NAME_SUR => Some(Country::SUR),
    NAME_SJM => Some(Country::SJM),
    NAME_SWE => Some(Country::SWE),
    NAME_CHE => Some(Country::CHE),
    NAME_SYR => Some(Country::SYR),
    NAME_TWN => Some(Country::TWN),
    NAME_TJK => Some(Country::TJK),
    NAME_TZA => Some(Country::TZA),
    NAME_THA => Some(Country::THA),
    NAME_TLS => Some(Country::TLS),
    NAME_TGO => Some(Country::TGO),
    NAME_TKL => Some(Country::TKL),
    NAME_TON => Some(Country::TON),
    NAME_TTO => Some(Country::TTO),
    NAME_TUN => Some(Country::TUN),
    NAME_TUR => Some(Country::TUR),
    NAME_TKM => Some(Country::TKM),
    NAME_TCA => Some(Country::TCA),
    NAME_TUV => Some(Country::TUV),
    NAME_UGA => Some(Country::UGA),
    NAME_UKR => Some(Country::UKR),
    NAME_ARE => Some(Country::ARE),
    NAME_GBR => Some(Country::GBR),
    NAME_USA => Some(Country::USA),
    NAME_UMI => Some(Country::UMI),
    NAME_URY => Some(Country::URY),
    NAME_UZB => Some(Country::UZB),
    NAME_VUT => Some(Country::VUT),
    NAME_VEN => Some(Country::VEN),
    NAME_VNM => Some(Country::VNM),
    NAME_VGB => Some(Country::VGB),
    NAME_VIR => Some(Country::VIR),
    NAME_WLF => Some(Country::WLF),
    NAME_ESH => Some(Country::ESH),
    NAME_YEM => Some(Country::YEM),
    NAME_ZMB => Some(Country::ZMB),
    NAME_ZWE => Some(Country::ZWE),
    _ => Option::None,
}
}
/// Constants for `alpha_2`
const ALPHA_2_AFG: &'static str = "AF";
const ALPHA_2_ALA: &'static str = "AX";
const ALPHA_2_ALB: &'static str = "AL";
const ALPHA_2_DZA: &'static str = "DZ";
const ALPHA_2_ASM: &'static str = "AS";
const ALPHA_2_AND: &'static str = "AD";
const ALPHA_2_AGO: &'static str = "AO";
const ALPHA_2_AIA: &'static str = "AI";
const ALPHA_2_ATA: &'static str = "AQ";
const ALPHA_2_ATG: &'static str = "AG";
const ALPHA_2_ARG: &'static str = "AR";
const ALPHA_2_ARM: &'static str = "AM";
const ALPHA_2_ABW: &'static str = "AW";
const ALPHA_2_AUS: &'static str = "AU";
const ALPHA_2_AUT: &'static str = "AT";
const ALPHA_2_AZE: &'static str = "AZ";
const ALPHA_2_BHS: &'static str = "BS";
const ALPHA_2_BHR: &'static str = "BH";
const ALPHA_2_BGD: &'static str = "BD";
const ALPHA_2_BRB: &'static str = "BB";
const ALPHA_2_BLR: &'static str = "BY";
const ALPHA_2_BEL: &'static str = "BE";
const ALPHA_2_BLZ: &'static str = "BZ";
const ALPHA_2_BEN: &'static str = "BJ";
const ALPHA_2_BMU: &'static str = "BM";
const ALPHA_2_BTN: &'static str = "BT";
const ALPHA_2_BOL: &'static str = "BO";
const ALPHA_2_BES: &'static str = "BQ";
const ALPHA_2_BIH: &'static str = "BA";
const ALPHA_2_BWA: &'static str = "BW";
const ALPHA_2_BVT: &'static str = "BV";
const ALPHA_2_BRA: &'static str = "BR";
const ALPHA_2_IOT: &'static str = "IO";
const ALPHA_2_BRN: &'static str = "BN";
const ALPHA_2_BGR: &'static str = "BG";
const ALPHA_2_BFA: &'static str = "BF";
const ALPHA_2_BDI: &'static str = "BI";
const ALPHA_2_CPV: &'static str = "CV";
const ALPHA_2_KHM: &'static str = "KH";
const ALPHA_2_CMR: &'static str = "CM";
const ALPHA_2_CAN: &'static str = "CA";
const ALPHA_2_CYM: &'static str = "KY";
const ALPHA_2_CAF: &'static str = "CF";
const ALPHA_2_TCD: &'static str = "TD";
const ALPHA_2_CHL: &'static str = "CL";
const ALPHA_2_CHN: &'static str = "CN";
const ALPHA_2_CXR: &'static str = "CX";
const ALPHA_2_CCK: &'static str = "CC";
const ALPHA_2_COL: &'static str = "CO";
const ALPHA_2_COM: &'static str = "KM";
const ALPHA_2_COG: &'static str = "CG";
const ALPHA_2_COD: &'static str = "CD";
const ALPHA_2_COK: &'static str = "CK";
const ALPHA_2_CRI: &'static str = "CR";
const ALPHA_2_CIV: &'static str = "CI";
const ALPHA_2_HRV: &'static str = "HR";
const ALPHA_2_CUB: &'static str = "CU";
const ALPHA_2_CUW: &'static str = "CW";
const ALPHA_2_CYP: &'static str = "CY";
const ALPHA_2_CZE: &'static str = "CZ";
const ALPHA_2_DNK: &'static str = "DK";
const ALPHA_2_DJI: &'static str = "DJ";
const ALPHA_2_DMA: &'static str = "DM";
const ALPHA_2_DOM: &'static str = "DO";
const ALPHA_2_ECU: &'static str = "EC";
const ALPHA_2_EGY: &'static str = "EG";
const ALPHA_2_SLV: &'static str = "SV";
const ALPHA_2_GNQ: &'static str = "GQ";
const ALPHA_2_ERI: &'static str = "ER";
const ALPHA_2_EST: &'static str = "EE";
const ALPHA_2_SWZ: &'static str = "SZ";
const ALPHA_2_ETH: &'static str = "ET";
const ALPHA_2_FLK: &'static str = "FK";
const ALPHA_2_FRO: &'static str = "FO";
const ALPHA_2_FJI: &'static str = "FJ";
const ALPHA_2_FIN: &'static str = "FI";
const ALPHA_2_FRA: &'static str = "FR";
const ALPHA_2_GUF: &'static str = "GF";
const ALPHA_2_PYF: &'static str = "PF";
const ALPHA_2_ATF: &'static str = "TF";
const ALPHA_2_GAB: &'static str = "GA";
const ALPHA_2_GMB: &'static str = "GM";
const ALPHA_2_GEO: &'static str = "GE";
const ALPHA_2_DEU: &'static str = "DE";
const ALPHA_2_GHA: &'static str = "GH";
const ALPHA_2_GIB: &'static str = "GI";
const ALPHA_2_GRC: &'static str = "GR";
const ALPHA_2_GRL: &'static str = "GL";
const ALPHA_2_GRD: &'static str = "GD";
const ALPHA_2_GLP: &'static str = "GP";
const ALPHA_2_GUM: &'static str = "GU";
const ALPHA_2_GTM: &'static str = "GT";
const ALPHA_2_GGY: &'static str = "GG";
const ALPHA_2_GIN: &'static str = "GN";
const ALPHA_2_GNB: &'static str = "GW";
const ALPHA_2_GUY: &'static str = "GY";
const ALPHA_2_HTI: &'static str = "HT";
const ALPHA_2_HMD: &'static str = "HM";
const ALPHA_2_VAT: &'static str = "VA";
const ALPHA_2_HND: &'static str = "HN";
const ALPHA_2_HKG: &'static str = "HK";
const ALPHA_2_HUN: &'static str = "HU";
const ALPHA_2_ISL: &'static str = "IS";
const ALPHA_2_IND: &'static str = "IN";
const ALPHA_2_IDN: &'static str = "ID";
const ALPHA_2_IRN: &'static str = "IR";
const ALPHA_2_IRQ: &'static str = "IQ";
const ALPHA_2_IRL: &'static str = "IE";
const ALPHA_2_IMN: &'static str = "IM";
const ALPHA_2_ISR: &'static str = "IL";
const ALPHA_2_ITA: &'static str = "IT";
const ALPHA_2_JAM: &'static str = "JM";
const ALPHA_2_JPN: &'static str = "JP";
const ALPHA_2_JEY: &'static str = "JE";
const ALPHA_2_JOR: &'static str = "JO";
const ALPHA_2_KAZ: &'static str = "KZ";
const ALPHA_2_KEN: &'static str = "KE";
const ALPHA_2_KIR: &'static str = "KI";
const ALPHA_2_PRK: &'static str = "KP";
const ALPHA_2_KOR: &'static str = "KR";
const ALPHA_2_KWT: &'static str = "KW";
const ALPHA_2_KGZ: &'static str = "KG";
const ALPHA_2_LAO: &'static str = "LA";
const ALPHA_2_LVA: &'static str = "LV";
const ALPHA_2_LBN: &'static str = "LB";
const ALPHA_2_LSO: &'static str = "LS";
const ALPHA_2_LBR: &'static str = "LR";
const ALPHA_2_LBY: &'static str = "LY";
const ALPHA_2_LIE: &'static str = "LI";
const ALPHA_2_LTU: &'static str = "LT";
const ALPHA_2_LUX: &'static str = "LU";
const ALPHA_2_MAC: &'static str = "MO";
const ALPHA_2_MDG: &'static str = "MG";
const ALPHA_2_MWI: &'static str = "MW";
const ALPHA_2_MYS: &'static str = "MY";
const ALPHA_2_MDV: &'static str = "MV";
const ALPHA_2_MLI: &'static str = "ML";
const ALPHA_2_MLT: &'static str = "MT";
const ALPHA_2_MHL: &'static str = "MH";
const ALPHA_2_MTQ: &'static str = "MQ";
const ALPHA_2_MRT: &'static str = "MR";
const ALPHA_2_MUS: &'static str = "MU";
const ALPHA_2_MYT: &'static str = "YT";
const ALPHA_2_MEX: &'static str = "MX";
const ALPHA_2_FSM: &'static str = "FM";
const ALPHA_2_MDA: &'static str = "MD";
const ALPHA_2_MCO: &'static str = "MC";
const ALPHA_2_MNG: &'static str = "MN";
const ALPHA_2_MNE: &'static str = "ME";
const ALPHA_2_MSR: &'static str = "MS";
const ALPHA_2_MAR: &'static str = "MA";
const ALPHA_2_MOZ: &'static str = "MZ";
const ALPHA_2_MMR: &'static str = "MM";
const ALPHA_2_NAM: &'static str = "NA";
const ALPHA_2_NRU: &'static str = "NR";
const ALPHA_2_NPL: &'static str = "NP";
const ALPHA_2_NLD: &'static str = "NL";
const ALPHA_2_NCL: &'static str = "NC";
const ALPHA_2_NZL: &'static str = "NZ";
const ALPHA_2_NIC: &'static str = "NI";
const ALPHA_2_NER: &'static str = "NE";
const ALPHA_2_NGA: &'static str = "NG";
const ALPHA_2_NIU: &'static str = "NU";
const ALPHA_2_NFK: &'static str = "NF";
const ALPHA_2_MKD: &'static str = "MK";
const ALPHA_2_MNP: &'static str = "MP";
const ALPHA_2_NOR: &'static str = "NO";
const ALPHA_2_OMN: &'static str = "OM";
const ALPHA_2_PAK: &'static str = "PK";
const ALPHA_2_PLW: &'static str = "PW";
const ALPHA_2_PSE: &'static str = "PS";
const ALPHA_2_PAN: &'static str = "PA";
const ALPHA_2_PNG: &'static str = "PG";
const ALPHA_2_PRY: &'static str = "PY";
const ALPHA_2_PER: &'static str = "PE";
const ALPHA_2_PHL: &'static str = "PH";
const ALPHA_2_PCN: &'static str = "PN";
const ALPHA_2_POL: &'static str = "PL";
const ALPHA_2_PRT: &'static str = "PT";
const ALPHA_2_PRI: &'static str = "PR";
const ALPHA_2_QAT: &'static str = "QA";
const ALPHA_2_REU: &'static str = "RE";
const ALPHA_2_ROU: &'static str = "RO";
const ALPHA_2_RUS: &'static str = "RU";
const ALPHA_2_RWA: &'static str = "RW";
const ALPHA_2_BLM: &'static str = "BL";
const ALPHA_2_SHN: &'static str = "SH";
const ALPHA_2_KNA: &'static str = "KN";
const ALPHA_2_LCA: &'static str = "LC";
const ALPHA_2_MAF: &'static str = "MF";
const ALPHA_2_SPM: &'static str = "PM";
const ALPHA_2_VCT: &'static str = "VC";
const ALPHA_2_WSM: &'static str = "WS";
const ALPHA_2_SMR: &'static str = "SM";
const ALPHA_2_STP: &'static str = "ST";
const ALPHA_2_SAU: &'static str = "SA";
const ALPHA_2_SEN: &'static str = "SN";
const ALPHA_2_SRB: &'static str = "RS";
const ALPHA_2_SYC: &'static str = "SC";
const ALPHA_2_SLE: &'static str = "SL";
const ALPHA_2_SGP: &'static str = "SG";
const ALPHA_2_SXM: &'static str = "SX";
const ALPHA_2_SVK: &'static str = "SK";
const ALPHA_2_SVN: &'static str = "SI";
const ALPHA_2_SLB: &'static str = "SB";
const ALPHA_2_SOM: &'static str = "SO";
const ALPHA_2_ZAF: &'static str = "ZA";
const ALPHA_2_SGS: &'static str = "GS";
const ALPHA_2_SSD: &'static str = "SS";
const ALPHA_2_ESP: &'static str = "ES";
const ALPHA_2_LKA: &'static str = "LK";
const ALPHA_2_SDN: &'static str = "SD";
const ALPHA_2_SUR: &'static str = "SR";
const ALPHA_2_SJM: &'static str = "SJ";
const ALPHA_2_SWE: &'static str = "SE";
const ALPHA_2_CHE: &'static str = "CH";
const ALPHA_2_SYR: &'static str = "SY";
const ALPHA_2_TWN: &'static str = "TW";
const ALPHA_2_TJK: &'static str = "TJ";
const ALPHA_2_TZA: &'static str = "TZ";
const ALPHA_2_THA: &'static str = "TH";
const ALPHA_2_TLS: &'static str = "TL";
const ALPHA_2_TGO: &'static str = "TG";
const ALPHA_2_TKL: &'static str = "TK";
const ALPHA_2_TON: &'static str = "TO";
const ALPHA_2_TTO: &'static str = "TT";
const ALPHA_2_TUN: &'static str = "TN";
const ALPHA_2_TUR: &'static str = "TR";
const ALPHA_2_TKM: &'static str = "TM";
const ALPHA_2_TCA: &'static str = "TC";
const ALPHA_2_TUV: &'static str = "TV";
const ALPHA_2_UGA: &'static str = "UG";
const ALPHA_2_UKR: &'static str = "UA";
const ALPHA_2_ARE: &'static str = "AE";
const ALPHA_2_GBR: &'static str = "GB";
const ALPHA_2_USA: &'static str = "US";
const ALPHA_2_UMI: &'static str = "UM";
const ALPHA_2_URY: &'static str = "UY";
const ALPHA_2_UZB: &'static str = "UZ";
const ALPHA_2_VUT: &'static str = "VU";
const ALPHA_2_VEN: &'static str = "VE";
const ALPHA_2_VNM: &'static str = "VN";
const ALPHA_2_VGB: &'static str = "VG";
const ALPHA_2_VIR: &'static str = "VI";
const ALPHA_2_WLF: &'static str = "WF";
const ALPHA_2_ESH: &'static str = "EH";
const ALPHA_2_YEM: &'static str = "YE";
const ALPHA_2_ZMB: &'static str = "ZM";
const ALPHA_2_ZWE: &'static str = "ZW";

/// Function to convert from Country to alpha_2
pub const fn country_as_alpha_2(country: &Country) -> &'static str
{
match country {
    Country::AFG => ALPHA_2_AFG,
    Country::ALA => ALPHA_2_ALA,
    Country::ALB => ALPHA_2_ALB,
    Country::DZA => ALPHA_2_DZA,
    Country::ASM => ALPHA_2_ASM,
    Country::AND => ALPHA_2_AND,
    Country::AGO => ALPHA_2_AGO,
    Country::AIA => ALPHA_2_AIA,
    Country::ATA => ALPHA_2_ATA,
    Country::ATG => ALPHA_2_ATG,
    Country::ARG => ALPHA_2_ARG,
    Country::ARM => ALPHA_2_ARM,
    Country::ABW => ALPHA_2_ABW,
    Country::AUS => ALPHA_2_AUS,
    Country::AUT => ALPHA_2_AUT,
    Country::AZE => ALPHA_2_AZE,
    Country::BHS => ALPHA_2_BHS,
    Country::BHR => ALPHA_2_BHR,
    Country::BGD => ALPHA_2_BGD,
    Country::BRB => ALPHA_2_BRB,
    Country::BLR => ALPHA_2_BLR,
    Country::BEL => ALPHA_2_BEL,
    Country::BLZ => ALPHA_2_BLZ,
    Country::BEN => ALPHA_2_BEN,
    Country::BMU => ALPHA_2_BMU,
    Country::BTN => ALPHA_2_BTN,
    Country::BOL => ALPHA_2_BOL,
    Country::BES => ALPHA_2_BES,
    Country::BIH => ALPHA_2_BIH,
    Country::BWA => ALPHA_2_BWA,
    Country::BVT => ALPHA_2_BVT,
    Country::BRA => ALPHA_2_BRA,
    Country::IOT => ALPHA_2_IOT,
    Country::BRN => ALPHA_2_BRN,
    Country::BGR => ALPHA_2_BGR,
    Country::BFA => ALPHA_2_BFA,
    Country::BDI => ALPHA_2_BDI,
    Country::CPV => ALPHA_2_CPV,
    Country::KHM => ALPHA_2_KHM,
    Country::CMR => ALPHA_2_CMR,
    Country::CAN => ALPHA_2_CAN,
    Country::CYM => ALPHA_2_CYM,
    Country::CAF => ALPHA_2_CAF,
    Country::TCD => ALPHA_2_TCD,
    Country::CHL => ALPHA_2_CHL,
    Country::CHN => ALPHA_2_CHN,
    Country::CXR => ALPHA_2_CXR,
    Country::CCK => ALPHA_2_CCK,
    Country::COL => ALPHA_2_COL,
    Country::COM => ALPHA_2_COM,
    Country::COG => ALPHA_2_COG,
    Country::COD => ALPHA_2_COD,
    Country::COK => ALPHA_2_COK,
    Country::CRI => ALPHA_2_CRI,
    Country::CIV => ALPHA_2_CIV,
    Country::HRV => ALPHA_2_HRV,
    Country::CUB => ALPHA_2_CUB,
    Country::CUW => ALPHA_2_CUW,
    Country::CYP => ALPHA_2_CYP,
    Country::CZE => ALPHA_2_CZE,
    Country::DNK => ALPHA_2_DNK,
    Country::DJI => ALPHA_2_DJI,
    Country::DMA => ALPHA_2_DMA,
    Country::DOM => ALPHA_2_DOM,
    Country::ECU => ALPHA_2_ECU,
    Country::EGY => ALPHA_2_EGY,
    Country::SLV => ALPHA_2_SLV,
    Country::GNQ => ALPHA_2_GNQ,
    Country::ERI => ALPHA_2_ERI,
    Country::EST => ALPHA_2_EST,
    Country::SWZ => ALPHA_2_SWZ,
    Country::ETH => ALPHA_2_ETH,
    Country::FLK => ALPHA_2_FLK,
    Country::FRO => ALPHA_2_FRO,
    Country::FJI => ALPHA_2_FJI,
    Country::FIN => ALPHA_2_FIN,
    Country::FRA => ALPHA_2_FRA,
    Country::GUF => ALPHA_2_GUF,
    Country::PYF => ALPHA_2_PYF,
    Country::ATF => ALPHA_2_ATF,
    Country::GAB => ALPHA_2_GAB,
    Country::GMB => ALPHA_2_GMB,
    Country::GEO => ALPHA_2_GEO,
    Country::DEU => ALPHA_2_DEU,
    Country::GHA => ALPHA_2_GHA,
    Country::GIB => ALPHA_2_GIB,
    Country::GRC => ALPHA_2_GRC,
    Country::GRL => ALPHA_2_GRL,
    Country::GRD => ALPHA_2_GRD,
    Country::GLP => ALPHA_2_GLP,
    Country::GUM => ALPHA_2_GUM,
    Country::GTM => ALPHA_2_GTM,
    Country::GGY => ALPHA_2_GGY,
    Country::GIN => ALPHA_2_GIN,
    Country::GNB => ALPHA_2_GNB,
    Country::GUY => ALPHA_2_GUY,
    Country::HTI => ALPHA_2_HTI,
    Country::HMD => ALPHA_2_HMD,
    Country::VAT => ALPHA_2_VAT,
    Country::HND => ALPHA_2_HND,
    Country::HKG => ALPHA_2_HKG,
    Country::HUN => ALPHA_2_HUN,
    Country::ISL => ALPHA_2_ISL,
    Country::IND => ALPHA_2_IND,
    Country::IDN => ALPHA_2_IDN,
    Country::IRN => ALPHA_2_IRN,
    Country::IRQ => ALPHA_2_IRQ,
    Country::IRL => ALPHA_2_IRL,
    Country::IMN => ALPHA_2_IMN,
    Country::ISR => ALPHA_2_ISR,
    Country::ITA => ALPHA_2_ITA,
    Country::JAM => ALPHA_2_JAM,
    Country::JPN => ALPHA_2_JPN,
    Country::JEY => ALPHA_2_JEY,
    Country::JOR => ALPHA_2_JOR,
    Country::KAZ => ALPHA_2_KAZ,
    Country::KEN => ALPHA_2_KEN,
    Country::KIR => ALPHA_2_KIR,
    Country::PRK => ALPHA_2_PRK,
    Country::KOR => ALPHA_2_KOR,
    Country::KWT => ALPHA_2_KWT,
    Country::KGZ => ALPHA_2_KGZ,
    Country::LAO => ALPHA_2_LAO,
    Country::LVA => ALPHA_2_LVA,
    Country::LBN => ALPHA_2_LBN,
    Country::LSO => ALPHA_2_LSO,
    Country::LBR => ALPHA_2_LBR,
    Country::LBY => ALPHA_2_LBY,
    Country::LIE => ALPHA_2_LIE,
    Country::LTU => ALPHA_2_LTU,
    Country::LUX => ALPHA_2_LUX,
    Country::MAC => ALPHA_2_MAC,
    Country::MDG => ALPHA_2_MDG,
    Country::MWI => ALPHA_2_MWI,
    Country::MYS => ALPHA_2_MYS,
    Country::MDV => ALPHA_2_MDV,
    Country::MLI => ALPHA_2_MLI,
    Country::MLT => ALPHA_2_MLT,
    Country::MHL => ALPHA_2_MHL,
    Country::MTQ => ALPHA_2_MTQ,
    Country::MRT => ALPHA_2_MRT,
    Country::MUS => ALPHA_2_MUS,
    Country::MYT => ALPHA_2_MYT,
    Country::MEX => ALPHA_2_MEX,
    Country::FSM => ALPHA_2_FSM,
    Country::MDA => ALPHA_2_MDA,
    Country::MCO => ALPHA_2_MCO,
    Country::MNG => ALPHA_2_MNG,
    Country::MNE => ALPHA_2_MNE,
    Country::MSR => ALPHA_2_MSR,
    Country::MAR => ALPHA_2_MAR,
    Country::MOZ => ALPHA_2_MOZ,
    Country::MMR => ALPHA_2_MMR,
    Country::NAM => ALPHA_2_NAM,
    Country::NRU => ALPHA_2_NRU,
    Country::NPL => ALPHA_2_NPL,
    Country::NLD => ALPHA_2_NLD,
    Country::NCL => ALPHA_2_NCL,
    Country::NZL => ALPHA_2_NZL,
    Country::NIC => ALPHA_2_NIC,
    Country::NER => ALPHA_2_NER,
    Country::NGA => ALPHA_2_NGA,
    Country::NIU => ALPHA_2_NIU,
    Country::NFK => ALPHA_2_NFK,
    Country::MKD => ALPHA_2_MKD,
    Country::MNP => ALPHA_2_MNP,
    Country::NOR => ALPHA_2_NOR,
    Country::OMN => ALPHA_2_OMN,
    Country::PAK => ALPHA_2_PAK,
    Country::PLW => ALPHA_2_PLW,
    Country::PSE => ALPHA_2_PSE,
    Country::PAN => ALPHA_2_PAN,
    Country::PNG => ALPHA_2_PNG,
    Country::PRY => ALPHA_2_PRY,
    Country::PER => ALPHA_2_PER,
    Country::PHL => ALPHA_2_PHL,
    Country::PCN => ALPHA_2_PCN,
    Country::POL => ALPHA_2_POL,
    Country::PRT => ALPHA_2_PRT,
    Country::PRI => ALPHA_2_PRI,
    Country::QAT => ALPHA_2_QAT,
    Country::REU => ALPHA_2_REU,
    Country::ROU => ALPHA_2_ROU,
    Country::RUS => ALPHA_2_RUS,
    Country::RWA => ALPHA_2_RWA,
    Country::BLM => ALPHA_2_BLM,
    Country::SHN => ALPHA_2_SHN,
    Country::KNA => ALPHA_2_KNA,
    Country::LCA => ALPHA_2_LCA,
    Country::MAF => ALPHA_2_MAF,
    Country::SPM => ALPHA_2_SPM,
    Country::VCT => ALPHA_2_VCT,
    Country::WSM => ALPHA_2_WSM,
    Country::SMR => ALPHA_2_SMR,
    Country::STP => ALPHA_2_STP,
    Country::SAU => ALPHA_2_SAU,
    Country::SEN => ALPHA_2_SEN,
    Country::SRB => ALPHA_2_SRB,
    Country::SYC => ALPHA_2_SYC,
    Country::SLE => ALPHA_2_SLE,
    Country::SGP => ALPHA_2_SGP,
    Country::SXM => ALPHA_2_SXM,
    Country::SVK => ALPHA_2_SVK,
    Country::SVN => ALPHA_2_SVN,
    Country::SLB => ALPHA_2_SLB,
    Country::SOM => ALPHA_2_SOM,
    Country::ZAF => ALPHA_2_ZAF,
    Country::SGS => ALPHA_2_SGS,
    Country::SSD => ALPHA_2_SSD,
    Country::ESP => ALPHA_2_ESP,
    Country::LKA => ALPHA_2_LKA,
    Country::SDN => ALPHA_2_SDN,
    Country::SUR => ALPHA_2_SUR,
    Country::SJM => ALPHA_2_SJM,
    Country::SWE => ALPHA_2_SWE,
    Country::CHE => ALPHA_2_CHE,
    Country::SYR => ALPHA_2_SYR,
    Country::TWN => ALPHA_2_TWN,
    Country::TJK => ALPHA_2_TJK,
    Country::TZA => ALPHA_2_TZA,
    Country::THA => ALPHA_2_THA,
    Country::TLS => ALPHA_2_TLS,
    Country::TGO => ALPHA_2_TGO,
    Country::TKL => ALPHA_2_TKL,
    Country::TON => ALPHA_2_TON,
    Country::TTO => ALPHA_2_TTO,
    Country::TUN => ALPHA_2_TUN,
    Country::TUR => ALPHA_2_TUR,
    Country::TKM => ALPHA_2_TKM,
    Country::TCA => ALPHA_2_TCA,
    Country::TUV => ALPHA_2_TUV,
    Country::UGA => ALPHA_2_UGA,
    Country::UKR => ALPHA_2_UKR,
    Country::ARE => ALPHA_2_ARE,
    Country::GBR => ALPHA_2_GBR,
    Country::USA => ALPHA_2_USA,
    Country::UMI => ALPHA_2_UMI,
    Country::URY => ALPHA_2_URY,
    Country::UZB => ALPHA_2_UZB,
    Country::VUT => ALPHA_2_VUT,
    Country::VEN => ALPHA_2_VEN,
    Country::VNM => ALPHA_2_VNM,
    Country::VGB => ALPHA_2_VGB,
    Country::VIR => ALPHA_2_VIR,
    Country::WLF => ALPHA_2_WLF,
    Country::ESH => ALPHA_2_ESH,
    Country::YEM => ALPHA_2_YEM,
    Country::ZMB => ALPHA_2_ZMB,
    Country::ZWE => ALPHA_2_ZWE,
}
}

/// Function to convert from alpha_2 to Country
pub fn country_from_alpha_2(alpha_2: &str) -> Option<Country>
{
match alpha_2 {
    ALPHA_2_AFG => Some(Country::AFG),
    ALPHA_2_ALA => Some(Country::ALA),
    ALPHA_2_ALB => Some(Country::ALB),
    ALPHA_2_DZA => Some(Country::DZA),
    ALPHA_2_ASM => Some(Country::ASM),
    ALPHA_2_AND => Some(Country::AND),
    ALPHA_2_AGO => Some(Country::AGO),
    ALPHA_2_AIA => Some(Country::AIA),
    ALPHA_2_ATA => Some(Country::ATA),
    ALPHA_2_ATG => Some(Country::ATG),
    ALPHA_2_ARG => Some(Country::ARG),
    ALPHA_2_ARM => Some(Country::ARM),
    ALPHA_2_ABW => Some(Country::ABW),
    ALPHA_2_AUS => Some(Country::AUS),
    ALPHA_2_AUT => Some(Country::AUT),
    ALPHA_2_AZE => Some(Country::AZE),
    ALPHA_2_BHS => Some(Country::BHS),
    ALPHA_2_BHR => Some(Country::BHR),
    ALPHA_2_BGD => Some(Country::BGD),
    ALPHA_2_BRB => Some(Country::BRB),
    ALPHA_2_BLR => Some(Country::BLR),
    ALPHA_2_BEL => Some(Country::BEL),
    ALPHA_2_BLZ => Some(Country::BLZ),
    ALPHA_2_BEN => Some(Country::BEN),
    ALPHA_2_BMU => Some(Country::BMU),
    ALPHA_2_BTN => Some(Country::BTN),
    ALPHA_2_BOL => Some(Country::BOL),
    ALPHA_2_BES => Some(Country::BES),
    ALPHA_2_BIH => Some(Country::BIH),
    ALPHA_2_BWA => Some(Country::BWA),
    ALPHA_2_BVT => Some(Country::BVT),
    ALPHA_2_BRA => Some(Country::BRA),
    ALPHA_2_IOT => Some(Country::IOT),
    ALPHA_2_BRN => Some(Country::BRN),
    ALPHA_2_BGR => Some(Country::BGR),
    ALPHA_2_BFA => Some(Country::BFA),
    ALPHA_2_BDI => Some(Country::BDI),
    ALPHA_2_CPV => Some(Country::CPV),
    ALPHA_2_KHM => Some(Country::KHM),
    ALPHA_2_CMR => Some(Country::CMR),
    ALPHA_2_CAN => Some(Country::CAN),
    ALPHA_2_CYM => Some(Country::CYM),
    ALPHA_2_CAF => Some(Country::CAF),
    ALPHA_2_TCD => Some(Country::TCD),
    ALPHA_2_CHL => Some(Country::CHL),
    ALPHA_2_CHN => Some(Country::CHN),
    ALPHA_2_CXR => Some(Country::CXR),
    ALPHA_2_CCK => Some(Country::CCK),
    ALPHA_2_COL => Some(Country::COL),
    ALPHA_2_COM => Some(Country::COM),
    ALPHA_2_COG => Some(Country::COG),
    ALPHA_2_COD => Some(Country::COD),
    ALPHA_2_COK => Some(Country::COK),
    ALPHA_2_CRI => Some(Country::CRI),
    ALPHA_2_CIV => Some(Country::CIV),
    ALPHA_2_HRV => Some(Country::HRV),
    ALPHA_2_CUB => Some(Country::CUB),
    ALPHA_2_CUW => Some(Country::CUW),
    ALPHA_2_CYP => Some(Country::CYP),
    ALPHA_2_CZE => Some(Country::CZE),
    ALPHA_2_DNK => Some(Country::DNK),
    ALPHA_2_DJI => Some(Country::DJI),
    ALPHA_2_DMA => Some(Country::DMA),
    ALPHA_2_DOM => Some(Country::DOM),
    ALPHA_2_ECU => Some(Country::ECU),
    ALPHA_2_EGY => Some(Country::EGY),
    ALPHA_2_SLV => Some(Country::SLV),
    ALPHA_2_GNQ => Some(Country::GNQ),
    ALPHA_2_ERI => Some(Country::ERI),
    ALPHA_2_EST => Some(Country::EST),
    ALPHA_2_SWZ => Some(Country::SWZ),
    ALPHA_2_ETH => Some(Country::ETH),
    ALPHA_2_FLK => Some(Country::FLK),
    ALPHA_2_FRO => Some(Country::FRO),
    ALPHA_2_FJI => Some(Country::FJI),
    ALPHA_2_FIN => Some(Country::FIN),
    ALPHA_2_FRA => Some(Country::FRA),
    ALPHA_2_GUF => Some(Country::GUF),
    ALPHA_2_PYF => Some(Country::PYF),
    ALPHA_2_ATF => Some(Country::ATF),
    ALPHA_2_GAB => Some(Country::GAB),
    ALPHA_2_GMB => Some(Country::GMB),
    ALPHA_2_GEO => Some(Country::GEO),
    ALPHA_2_DEU => Some(Country::DEU),
    ALPHA_2_GHA => Some(Country::GHA),
    ALPHA_2_GIB => Some(Country::GIB),
    ALPHA_2_GRC => Some(Country::GRC),
    ALPHA_2_GRL => Some(Country::GRL),
    ALPHA_2_GRD => Some(Country::GRD),
    ALPHA_2_GLP => Some(Country::GLP),
    ALPHA_2_GUM => Some(Country::GUM),
    ALPHA_2_GTM => Some(Country::GTM),
    ALPHA_2_GGY => Some(Country::GGY),
    ALPHA_2_GIN => Some(Country::GIN),
    ALPHA_2_GNB => Some(Country::GNB),
    ALPHA_2_GUY => Some(Country::GUY),
    ALPHA_2_HTI => Some(Country::HTI),
    ALPHA_2_HMD => Some(Country::HMD),
    ALPHA_2_VAT => Some(Country::VAT),
    ALPHA_2_HND => Some(Country::HND),
    ALPHA_2_HKG => Some(Country::HKG),
    ALPHA_2_HUN => Some(Country::HUN),
    ALPHA_2_ISL => Some(Country::ISL),
    ALPHA_2_IND => Some(Country::IND),
    ALPHA_2_IDN => Some(Country::IDN),
    ALPHA_2_IRN => Some(Country::IRN),
    ALPHA_2_IRQ => Some(Country::IRQ),
    ALPHA_2_IRL => Some(Country::IRL),
    ALPHA_2_IMN => Some(Country::IMN),
    ALPHA_2_ISR => Some(Country::ISR),
    ALPHA_2_ITA => Some(Country::ITA),
    ALPHA_2_JAM => Some(Country::JAM),
    ALPHA_2_JPN => Some(Country::JPN),
    ALPHA_2_JEY => Some(Country::JEY),
    ALPHA_2_JOR => Some(Country::JOR),
    ALPHA_2_KAZ => Some(Country::KAZ),
    ALPHA_2_KEN => Some(Country::KEN),
    ALPHA_2_KIR => Some(Country::KIR),
    ALPHA_2_PRK => Some(Country::PRK),
    ALPHA_2_KOR => Some(Country::KOR),
    ALPHA_2_KWT => Some(Country::KWT),
    ALPHA_2_KGZ => Some(Country::KGZ),
    ALPHA_2_LAO => Some(Country::LAO),
    ALPHA_2_LVA => Some(Country::LVA),
    ALPHA_2_LBN => Some(Country::LBN),
    ALPHA_2_LSO => Some(Country::LSO),
    ALPHA_2_LBR => Some(Country::LBR),
    ALPHA_2_LBY => Some(Country::LBY),
    ALPHA_2_LIE => Some(Country::LIE),
    ALPHA_2_LTU => Some(Country::LTU),
    ALPHA_2_LUX => Some(Country::LUX),
    ALPHA_2_MAC => Some(Country::MAC),
    ALPHA_2_MDG => Some(Country::MDG),
    ALPHA_2_MWI => Some(Country::MWI),
    ALPHA_2_MYS => Some(Country::MYS),
    ALPHA_2_MDV => Some(Country::MDV),
    ALPHA_2_MLI => Some(Country::MLI),
    ALPHA_2_MLT => Some(Country::MLT),
    ALPHA_2_MHL => Some(Country::MHL),
    ALPHA_2_MTQ => Some(Country::MTQ),
    ALPHA_2_MRT => Some(Country::MRT),
    ALPHA_2_MUS => Some(Country::MUS),
    ALPHA_2_MYT => Some(Country::MYT),
    ALPHA_2_MEX => Some(Country::MEX),
    ALPHA_2_FSM => Some(Country::FSM),
    ALPHA_2_MDA => Some(Country::MDA),
    ALPHA_2_MCO => Some(Country::MCO),
    ALPHA_2_MNG => Some(Country::MNG),
    ALPHA_2_MNE => Some(Country::MNE),
    ALPHA_2_MSR => Some(Country::MSR),
    ALPHA_2_MAR => Some(Country::MAR),
    ALPHA_2_MOZ => Some(Country::MOZ),
    ALPHA_2_MMR => Some(Country::MMR),
    ALPHA_2_NAM => Some(Country::NAM),
    ALPHA_2_NRU => Some(Country::NRU),
    ALPHA_2_NPL => Some(Country::NPL),
    ALPHA_2_NLD => Some(Country::NLD),
    ALPHA_2_NCL => Some(Country::NCL),
    ALPHA_2_NZL => Some(Country::NZL),
    ALPHA_2_NIC => Some(Country::NIC),
    ALPHA_2_NER => Some(Country::NER),
    ALPHA_2_NGA => Some(Country::NGA),
    ALPHA_2_NIU => Some(Country::NIU),
    ALPHA_2_NFK => Some(Country::NFK),
    ALPHA_2_MKD => Some(Country::MKD),
    ALPHA_2_MNP => Some(Country::MNP),
    ALPHA_2_NOR => Some(Country::NOR),
    ALPHA_2_OMN => Some(Country::OMN),
    ALPHA_2_PAK => Some(Country::PAK),
    ALPHA_2_PLW => Some(Country::PLW),
    ALPHA_2_PSE => Some(Country::PSE),
    ALPHA_2_PAN => Some(Country::PAN),
    ALPHA_2_PNG => Some(Country::PNG),
    ALPHA_2_PRY => Some(Country::PRY),
    ALPHA_2_PER => Some(Country::PER),
    ALPHA_2_PHL => Some(Country::PHL),
    ALPHA_2_PCN => Some(Country::PCN),
    ALPHA_2_POL => Some(Country::POL),
    ALPHA_2_PRT => Some(Country::PRT),
    ALPHA_2_PRI => Some(Country::PRI),
    ALPHA_2_QAT => Some(Country::QAT),
    ALPHA_2_REU => Some(Country::REU),
    ALPHA_2_ROU => Some(Country::ROU),
    ALPHA_2_RUS => Some(Country::RUS),
    ALPHA_2_RWA => Some(Country::RWA),
    ALPHA_2_BLM => Some(Country::BLM),
    ALPHA_2_SHN => Some(Country::SHN),
    ALPHA_2_KNA => Some(Country::KNA),
    ALPHA_2_LCA => Some(Country::LCA),
    ALPHA_2_MAF => Some(Country::MAF),
    ALPHA_2_SPM => Some(Country::SPM),
    ALPHA_2_VCT => Some(Country::VCT),
    ALPHA_2_WSM => Some(Country::WSM),
    ALPHA_2_SMR => Some(Country::SMR),
    ALPHA_2_STP => Some(Country::STP),
    ALPHA_2_SAU => Some(Country::SAU),
    ALPHA_2_SEN => Some(Country::SEN),
    ALPHA_2_SRB => Some(Country::SRB),
    ALPHA_2_SYC => Some(Country::SYC),
    ALPHA_2_SLE => Some(Country::SLE),
    ALPHA_2_SGP => Some(Country::SGP),
    ALPHA_2_SXM => Some(Country::SXM),
    ALPHA_2_SVK => Some(Country::SVK),
    ALPHA_2_SVN => Some(Country::SVN),
    ALPHA_2_SLB => Some(Country::SLB),
    ALPHA_2_SOM => Some(Country::SOM),
    ALPHA_2_ZAF => Some(Country::ZAF),
    ALPHA_2_SGS => Some(Country::SGS),
    ALPHA_2_SSD => Some(Country::SSD),
    ALPHA_2_ESP => Some(Country::ESP),
    ALPHA_2_LKA => Some(Country::LKA),
    ALPHA_2_SDN => Some(Country::SDN),
    ALPHA_2_SUR => Some(Country::SUR),
    ALPHA_2_SJM => Some(Country::SJM),
    ALPHA_2_SWE => Some(Country::SWE),
    ALPHA_2_CHE => Some(Country::CHE),
    ALPHA_2_SYR => Some(Country::SYR),
    ALPHA_2_TWN => Some(Country::TWN),
    ALPHA_2_TJK => Some(Country::TJK),
    ALPHA_2_TZA => Some(Country::TZA),
    ALPHA_2_THA => Some(Country::THA),
    ALPHA_2_TLS => Some(Country::TLS),
    ALPHA_2_TGO => Some(Country::TGO),
    ALPHA_2_TKL => Some(Country::TKL),
    ALPHA_2_TON => Some(Country::TON),
    ALPHA_2_TTO => Some(Country::TTO),
    ALPHA_2_TUN => Some(Country::TUN),
    ALPHA_2_TUR => Some(Country::TUR),
    ALPHA_2_TKM => Some(Country::TKM),
    ALPHA_2_TCA => Some(Country::TCA),
    ALPHA_2_TUV => Some(Country::TUV),
    ALPHA_2_UGA => Some(Country::UGA),
    ALPHA_2_UKR => Some(Country::UKR),
    ALPHA_2_ARE => Some(Country::ARE),
    ALPHA_2_GBR => Some(Country::GBR),
    ALPHA_2_USA => Some(Country::USA),
    ALPHA_2_UMI => Some(Country::UMI),
    ALPHA_2_URY => Some(Country::URY),
    ALPHA_2_UZB => Some(Country::UZB),
    ALPHA_2_VUT => Some(Country::VUT),
    ALPHA_2_VEN => Some(Country::VEN),
    ALPHA_2_VNM => Some(Country::VNM),
    ALPHA_2_VGB => Some(Country::VGB),
    ALPHA_2_VIR => Some(Country::VIR),
    ALPHA_2_WLF => Some(Country::WLF),
    ALPHA_2_ESH => Some(Country::ESH),
    ALPHA_2_YEM => Some(Country::YEM),
    ALPHA_2_ZMB => Some(Country::ZMB),
    ALPHA_2_ZWE => Some(Country::ZWE),
    _ => Option::None,
}
}
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_country_do_a_barrel_roll()
    {
        let country = Country::AFG;
        let result = country.as_country_code();
        let result = Country::from_country_code(result).unwrap();
        let result = country.as_name();
        let result = Country::from_name(result).unwrap();
        let result = country.as_alpha_2();
        let result = Country::from_alpha_2(result).unwrap();
        assert_eq!(country, result);
    }
}

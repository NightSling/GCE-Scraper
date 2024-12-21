use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub download_markscheme: bool,
    pub download_paper: bool,
    pub download_examiners_report: bool,
    pub subjects: Vec<YearConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearConfiguration {
    pub syllabus_code: SyllabusCode,
    pub years: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyllabusCode {
    pub name: String,
    pub syllabus_code: String,
    pub access_slug: String,
}


impl SyllabusCode {
    pub fn new(name: &str, access_slug: &str, syllabus_code: &str) -> Self {
        Self {
            name: name.to_string(),
            syllabus_code: syllabus_code.to_string(),
            access_slug: access_slug.to_string(),
        }
    }
}

pub static SYLLABUS_CODES: LazyLock<Vec<SyllabusCode>> = LazyLock::new(|| {
    vec!(
        SyllabusCode::new("Accounting", "accounting-(9706)", "9706"),
        SyllabusCode::new("Afrikaans", "afrikaans-(9679)", "9679"),
        SyllabusCode::new(
            "Applied Information And Communication Technology",
            "applied-Information-and-communication-technology-(9713)",
            "9713",
        ),
        SyllabusCode::new("Arabic", "arabic-(9680)", "9680"),
        SyllabusCode::new(
            "Arabic Language",
            "arabic-language-(AS-level-only)-(8680)",
            "8680",
        ),
        SyllabusCode::new("Art & Design", "art-&-design-(9479)", "9479"),
        SyllabusCode::new("Art & Design", "art-&-design-(9704)", "9704"),
        SyllabusCode::new("Biblical Studies", "biblical-studies-(9484)", "9484"),
        SyllabusCode::new("Biology", "biology-(9700)", "9700"),
        SyllabusCode::new("Business", "business-(9609)", "9609"),
        SyllabusCode::new("Business Studies", "business-studies-(9707)", "9707"),
        SyllabusCode::new(
            "Cambridge International Project Qualification",
            "cambridge-international-project-qualification-(9980)",
            "9980",
        ),
        SyllabusCode::new("Chemistry", "chemistry-(9701)", "9701"),
        SyllabusCode::new("Chinese", "chinese-(A level only)-(9715)", "9715"),
        SyllabusCode::new(
            "Chinese Language",
            "chinese-language-(AS-level-only)-(8681)",
            "8681",
        ),
        SyllabusCode::new("Classical Studies", "classical-studies-(9274)", "9274"),
        SyllabusCode::new("Computer Science", "computer-science-(9608)", "9608"),
        SyllabusCode::new("Computer Science", "computer-science-(9618)", "9618"),
        SyllabusCode::new("Computing", "computing-(9691)", "9691"),
        SyllabusCode::new("Design & Technology", "design-&-technology-(9705)", "9705"),
        SyllabusCode::new("Design & Textiles", "design-&-textiles-(9631)", "9631"),
        SyllabusCode::new(
            "Digital Media & Design",
            "digital-media-&-design-(9481)",
            "9481",
        ),
        SyllabusCode::new("Divinity", "divinity-(9011)", "9011"),
        SyllabusCode::new("Divinity", "divinity-(AS-level-only)-(8041)", "8041"),
        SyllabusCode::new("Drama", "drama-(9482)", "9482"),
        SyllabusCode::new("Economics", "economics-(9708)", "9708"),
        SyllabusCode::new(
            "English General Paper",
            "english-general-paper-(AS-level-only)-(8021)",
            "8021",
        ),
        SyllabusCode::new(
            "English Language & Literature",
            "english-language-&-literature-(AS-level-only)-(8695)",
            "8695",
        ),
        SyllabusCode::new("English Language", "english-language-(9093)", "9093"),
        SyllabusCode::new("English Literature", "english-literature-(9695)", "9695"),
        SyllabusCode::new(
            "Environmental Management",
            "environmental-management-(AS-only)-(8291)",
            "8291",
        ),
        SyllabusCode::new("Food Studies", "food-studies-(9336)", "9336"),
        SyllabusCode::new("French", "french-(A-level-only)-(9716)", "9716"),
        SyllabusCode::new(
            "French Language",
            "french-language-(AS-level-only)-(8682)",
            "8682",
        ),
        SyllabusCode::new(
            "French Literature",
            "french-literature-(AS-level-only)-(8670)",
            "8670",
        ),
        SyllabusCode::new(
            "General Paper",
            "general-paper-(AS-level-only)-(8001)",
            "8001",
        ),
        SyllabusCode::new(
            "General Paper",
            "general-paper-(AS-level-only)-(8004)",
            "8004",
        ),
        SyllabusCode::new("Geography", "geography-(9696)", "9696"),
        SyllabusCode::new("German", "german-(A-level-only)-(9717)", "9717"),
        SyllabusCode::new(
            "German Language",
            "german-language-(AS-level-only)-(8683)",
            "8683",
        ),
        SyllabusCode::new(
            "Global Perspectives & Research",
            "global-perspectives-&-research-(9239)",
            "9239",
        ),
        SyllabusCode::new("Hindi", "hindi-(A-level-only)-(9687)", "9687"),
        SyllabusCode::new(
            "Hindi Language",
            "hindi-language-(AS-level-only)-(8687)",
            "8687",
        ),
        SyllabusCode::new(
            "Hindi Literature",
            "hindi-literature-(AS-level-only)-(8675)",
            "8675",
        ),
        SyllabusCode::new("Hinduism", "hinduism-(9014)", "9014"),
        SyllabusCode::new("Hinduism", "hinduism-(9487)", "9487"),
        SyllabusCode::new("Hinduism", "hinduism-(AS-level-only)-(8058)", "8058"),
        SyllabusCode::new("History", "history-(9389)", "9389"),
        SyllabusCode::new("History", "history-(9489)", "9489"),
        SyllabusCode::new(
            "Information Technology",
            "information-technology-(9626)",
            "9626",
        ),
        SyllabusCode::new("Islamic Studies", "islamic-studies-(9013)", "9013"),
        SyllabusCode::new("Islamic Studies", "islamic-studies-(9013-&-8053)", "9013"),
        SyllabusCode::new("Islamic Studies", "islamic-studies-(9488)", "9488"),
        SyllabusCode::new(
            "Islamic Studies",
            "islamic-studies-(AS-level-only)-(8053)",
            "8053",
        ),
        SyllabusCode::new(
            "Japanese Language",
            "japanese-language-(AS-level-only)-(8281)",
            "8281",
        ),
        SyllabusCode::new("Law", "law-(9084)", "9084"),
        SyllabusCode::new("Marine Science", "marine-science-(9693)", "9693"),
        SyllabusCode::new("Mathematics", "mathematics-(9709)", "9709"),
        SyllabusCode::new("Mathematics Further", "mathematics-further-(9231)", "9231"),
        SyllabusCode::new("Media Studies", "media-studies-(9607)", "9607"),
        SyllabusCode::new("Music", "music-(9483)", "9483"),
        SyllabusCode::new("Music", "music-(9703)", "9703"),
        SyllabusCode::new("Music", "music-(AS-level-only)-(8663)", "8663"),
        SyllabusCode::new(
            "Nepal Studies",
            "nepal-studies(AS-level-only)-(8024)",
            "8024",
        ),
        SyllabusCode::new("Physical Education", "physical-education-(9396)", "9396"),
        SyllabusCode::new("Physics", "physics-(9702)", "9702"),
        SyllabusCode::new("Portuguese", "portuguese-(A-level-only)-(9718)", "9718"),
        SyllabusCode::new(
            "Portuguese Language",
            "portuguese-language-(AS-level-only)-(8684)",
            "8684",
        ),
        SyllabusCode::new(
            "Portuguese Literature",
            "portuguese-literature-(AS-level-only)-(8672)",
            "8672",
        ),
        SyllabusCode::new("Psychology", "psychology-(9698)", "9698"),
        SyllabusCode::new("Psychology", "psychology-(9990)", "9990"),
        SyllabusCode::new("Sociology", "sociology-(9699)", "9699"),
        SyllabusCode::new("Spanish", "spanish-(A-level-only)-(9719)", "9719"),
        SyllabusCode::new(
            "Spanish First Language",
            "spanish-first-language-(AS-level-only)-(8665)",
            "8665",
        ),
        SyllabusCode::new(
            "Spanish Language",
            "spanish-language-(AS-level-only)-(8685)",
            "8685",
        ),
        SyllabusCode::new(
            "Spanish Literature",
            "spanish-literature-(AS-level-only)-(8673)",
            "8673",
        ),
        SyllabusCode::new("Tamil", "tamil-(9689)", "9689"),
        SyllabusCode::new(
            "Tamil Language",
            "tamil-language-(AS-level-only)-(8689)",
            "8689",
        ),
        SyllabusCode::new("Thinking Skills", "thinking-skills-(9694)", "9694"),
        SyllabusCode::new("Travel & Tourism", "travel-&-tourism-(9395)", "9395"),
        SyllabusCode::new("Urdu", "urdu-(A-level-only)-(9676)", "9676"),
        SyllabusCode::new(
            "Urdu Language",
            "urdu-language-(AS-level-only)-(8686)",
            "8686",
        ),
        SyllabusCode::new(
            "Urdu Pakistan Only",
            "urdu-pakistan-only-(A-level-only)-(9686)",
            "9686",
        ),
    )
});

#!/bin/bash

echo "=== English Detection Benchmark ==="
echo "Configuration: 4 languages (English, French, German, Spanish) + min_distance=0.4 + confidence>0.5"
echo ""

test_text() {
    local label="$1"
    local text="$2"
    result=$(curl -s -X POST http://localhost:3030/detect \
        -H "Content-Type: application/json" \
        -d "{\"texts\":[\"$text\"]}")

    is_en=$(echo "$result" | jq -r '.results[0].is_english')

    printf "| %-25s | %-5s |\n" "\"$text\"" "$is_en"
}

echo "| Test Text                | is_en |"
echo "|--------------------------|-------|"

echo "| **English Tests**        |       |       |"
test_text "the" "the"
test_text "programming" "programming"
test_text "I love programming" "I love programming"
test_text "Medium English" "Machine learning is a subset of artificial intelligence that enables computers to learn without being explicitly programmed"
test_text "Long English text" "This is a comprehensive explanation of machine learning algorithms and their applications in modern software development practices"

echo "| **4-Language Set**       |       |       |"
test_text "Bonjour (French)" "Bonjour"
test_text "Guten Tag (German)" "Guten Tag"
test_text "Hola (Spanish)" "Hola"
test_text "Medium Spanish" "Soy un desarrollador de software y me gusta trabajar con tecnologías modernas"
test_text "Medium French" "Je suis développeur de logiciels et j'aime travailler avec des technologies modernes"
test_text "Medium German" "Ich bin Softwareentwickler und arbeite gerne mit modernen Technologien"

echo "| **Other European**       |       |       |"
test_text "Ciao (Italian)" "Ciao"
test_text "Medium Italian" "Sono uno sviluppatore di software e mi piace lavorare con tecnologie moderne"
test_text "Hallo (Dutch)" "Hallo"
test_text "Medium Dutch" "Ik ben een softwareontwikkelaar en ik werk graag met moderne technologieën"
test_text "Hej (Swedish)" "Hej"
test_text "Medium Swedish" "Jag är en mjukvaruutvecklare och jag gillar att arbeta med moderna teknologier"
test_text "Привет (Russian)" "Привет"
test_text "Medium Russian" "Я разработчик программного обеспечения и люблю работать с современными технологиями"
test_text "Cześć (Polish)" "Cześć"
test_text "Medium Polish" "Jestem programistą i lubię pracować z nowoczesnymi technologiami"

echo "| **Asian Languages**      |       |       |"
test_text "こんにちは (Japanese)" "こんにちは"
test_text "Medium Japanese" "私はソフトウェア開発者で、最新のテクノロジーで働くのが好きです"
test_text "안녕하세요 (Korean)" "안녕하세요"
test_text "Medium Korean" "저는 소프트웨어 개발자이고 현대 기술로 일하는 것을 좋아합니다"
test_text "नमस्ते (Hindi)" "नमस्ते"
test_text "Medium Hindi" "मैं एक सॉफ्टवेयर डेवलपर हूं और आधुनिक तकनीकों के साथ काम करना पसंद करता हूं"
test_text "สวัสดี (Thai)" "สวัสดี"
test_text "Medium Thai" "ฉันเป็นนักพัฒนาซอฟต์แวร์และชอบทำงานกับเทคโนโลยีสมัยใหม่"

echo "| **Middle East/Africa**   |       |       |"
test_text "مرحبا (Arabic)" "مرحبا"
test_text "Medium Arabic" "أنا مطور برمجيات وأحب العمل مع التقنيات الحديثة"
test_text "שלום (Hebrew)" "שלום"
test_text "Medium Hebrew" "אני מפתח תוכנה ואני אוהב לעבוד עם טכנולוגיות מודרניות"
test_text "merhaba (Turkish)" "merhaba"
test_text "Medium Turkish" "Yazılım geliştiricisiyim ve modern teknolojilerle çalışmayı seviyorum"
test_text "sawubona (Zulu)" "sawubona"
test_text "Medium Zulu" "Ngiyisiphakeli se-software futhi ngiyathanda ukusebenza nobuchwepheshe besimanje"

echo "| **Ambiguous/Short**      |       |       |"
test_text "hello" "hello"
test_text "hi" "hi"
test_text "TIL" "TIL"
test_text "AITA" "AITA"
# STANDARDIZE AND BRAND DISPLAY LINE ONE

## STANDARD ALL TERMINAL SUPPORTED UTF ICONS

browser                 Ƅ       LATIN CAPITAL LETTER TONE SIX
claude_agent            Ⲵ       COPTIC CAPITAL LETTER OLD COPTIN AIN
config                  ⚙       GEAR
database                ⛁       WHITE DRAUGHTS KING
fetch                   ⚚       STAFF OF HERMES
filesystem              ⚒       HAMMER AND PICK
introspection           ⚝       STAR WITH INSIDE LINES
git                     ⛙       WHITE LEFT LANE MERGE
github                  ⇅       UTF 113
memory                  ⚿       SQUARED KEY
process                 ♆       NEPTUNE
prompt                  ⚑       BLACK FLAG
reasoner                ☫       FARSI SYMBOL
scrape_url              ☄       COMET
sequential_thinking     ⚛       ATOM        
terminal                ⛩       SHINTO SHRINE
web_search              ⚶       VESTA

### ICONS AS SCHEMA DATA

/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
will define an icon for all tool categories and (optionally) individual tools

## DISPLAY LINE ONE

- kodegen ⓚ symbol in ansi color 132 Hex: #af5f87
- 3 spaces 
- icon + tool name in ansi color 32 Hex: #0087d7
- 3 spaces
- timing in seconds (ceil)
  - green for success ansi color 35 Hex: #00af5f
  - yellow for pending ansi color 178 Hex: #d7af00
  - red for error ansi color 204 Hex: #ff5f87
  - seconds must be >= 1s || ceil() ensures this
  
### Display Line 1 Examples

ⓚ    Ⲵ claude_agent     118s
ⓚ    ⚚ fetch     17s
ⓚ    ⚒ fs_read_file     1s
ⓚ    ⛙ git_checkout     12s
ⓚ    ⚛ sequential_thinking     6s
ⓚ    ⛩ terminal     60s
ⓚ    ⚶ web search     9s

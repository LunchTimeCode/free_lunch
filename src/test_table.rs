use maud::{Markup, html};

const ACTIVE_TABLE_STYLE: &str = r###"{
  "borderRadius": "10px",
  "boxShadow": "rgb(172 172 172 / 17%) 0px 0.5px 1px 0px",
  "width":"100%",
  "background":"black"
  }"###;
const ACTIVE_TABLE_CELLSTYLE: &str = r###"
  {"color":"white"}
  "###;

const ACTIVE_TABLE_RESIZE_COLORS: &str = r###"{"click": "#727272"}"###;

const ACTIVE_TABLE_HEADER_STYLE: &str = r###"
  {"default": {"backgroundColor": "#2d2d2d"}, "hoverColors": {"backgroundColor": "#353535"}}
    "###;

const ACTIVE_TABLE_ROW_DROPDOWN: &str = r###"
{"displaySettings": {"isAvailable": false}}
      "###;

const ACTIVE_TABLE_CUSTOM_COL_SETTING: &str = r###"
  {
      "isSortAvailable": true,
      "isDeleteAvailable": false,
      "isInsertLeftAvailable": false,
      "isInsertRightAvailable": false,
      "isMoveAvailable": true
    }  "###;

const FILTER: &str = r###"
    {
        "styles": {
          "input": {"height": "22px", "fontSize": "15px", "width": "200px", "borderColor": "#818181", "color": "white", "backgroundColor": "black"},
          "placeholderColor": "white",
          "caseIcon": {
            "default": {"color": "white", "fontSize": "15px", "backgroundColor": "black"},
            "hover": {"color": "white"},
            "click": {"color": "white"},
            "active": {"color": "pink"}
          },
          "dropdownIcon": {
            "default": {"color": "white", "width": "18px", "height": "18px" },
            "hover": {"color": "grey"},
            "click": {"color": "grey"},
            "active": {"color": "pink"}
          }
        }
      }
    
    "###;
const ACTIVE_TABLE_COL_TYPES: &str = r###"["Text"]"###;
const ACTIVE_TABLE_DATA: &str = r###"
[
  ["Planet", "Diameter", "Mass", "Moons","Density"],
  ["Earth", 12756, 5.97, 1, 5514],
  ["Mars", 6792, 0.642, 2, 3934],
  ["Saturn", 120536, 568, 82, 687],
  ["Neptune", 49528, 102, 14, 1638]]
  "###;

pub fn table() -> Markup {
    html! {
        active-table
            filter=(FILTER)
            isCellTextEditable="false"
            displayAddNewColumn="false"
            displayAddNewRow="false"
            displayIndexColumn="false"
            rowDropdown=(ACTIVE_TABLE_ROW_DROPDOWN)
            availableDefaultColumnTypes=(ACTIVE_TABLE_COL_TYPES)
            columnDropdown=(ACTIVE_TABLE_CUSTOM_COL_SETTING)
            columnResizerColors = (ACTIVE_TABLE_RESIZE_COLORS)
            headerStyles=(ACTIVE_TABLE_HEADER_STYLE)
            cellStyle=(ACTIVE_TABLE_CELLSTYLE)
            tableStyle=(ACTIVE_TABLE_STYLE)
            data=(ACTIVE_TABLE_DATA)
        {

        }
    }
}

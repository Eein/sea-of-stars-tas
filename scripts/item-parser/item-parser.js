const fs = require('node:fs')

const locale_kv = require('./locale/Inventory-CAB-18cc0a97ed3fba0abaeabffc8cbc6c92--8004240389874202174.json')
const locale_definition = require('./locale/Inventory_EN-CAB-18cc0a97ed3fba0abaeabffc8cbc6c92--2619762323296537302.json')

let pairs = locale_kv.locIndexByLocStringId.keys.Array.map((val, i) => {
  return [val, locale_kv.locIndexByLocStringId.values.Array[i]]
})
const locale = Object.fromEntries(pairs)

const getFromLocale = (name) => {
  let idx = locale[name]
  return locale_definition.strings.Array[idx]
}

const formatEquippableBy = (num) => {
  if (num == undefined ) { return "None" }

  let characters = []

  if (num >> 0 & 1) { characters.push("PlayerPartyCharacter::Zale") }  
  if (num >> 1 & 1) { characters.push("PlayerPartyCharacter::Valere") }  
  if (num >> 2 & 1) { characters.push("PlayerPartyCharacter::Garl") }  
  if (num >> 3 & 1) { characters.push("PlayerPartyCharacter::Serai") }  
  if (num >> 4 & 1) { characters.push("PlayerPartyCharacter::Reshan") }  
  if (num >> 5 & 1) { characters.push("PlayerPartyCharacter::Bst") }  

  if (characters.length == 0) { return "None" }

  let out = "&[\r\n"
  characters.forEach((c) => {
    out += `    ${c},\r\n`
  })
  out += "  ]"

  return `Some(${out})`
}

fs.readdir('./input/', function(err, filenames) {
  if (err) {
    onError(err);
    return;
  }

  filenames.forEach(function(filename) {
    if (!filename.endsWith('.json')) { return }

    fs.readFile(`./input/${filename}`, 'utf8', (err, data) => {
      if (err) {
        console.error(err)
        return
      }
      let json = JSON.parse(data)
      let guid = json?.guid
      if (guid == undefined) { 
        // These are snacks and stuff, skip!
        return
      }
      let maxQuantity = json?.maxQuantity
      let orderPriority = json?.orderPriority
      let buyPrice = json?.buyPrice
      let sellPrice = json?.sellPrice
      let validCharacters = json?.validCharacters
      let isGroupTrinket = json?.isGroupTrinket
      let physicalAttack = json?.physicalAttack || 0
      let magicalAttack = json?.magicalAttack || 0
      // Temporary = pull in localization file
      let localeKey = json?.nameLocalizationId?.locId
      let name = getFromLocale(json?.nameLocalizationId?.locId)
      let itemType = localeKey?.split("_")[0]

      let out = ` 
// ${localeKey}
m.insert("${name}", Item {
  guid: "${guid}",
  item_type: ItemType::${itemType},
  name: "${name}",
  order_priority: ${orderPriority},
  max_quantity: ${maxQuantity},
  buy_price: ${buyPrice},
  sell_price: ${sellPrice},
  equippable_by: ${formatEquippableBy(validCharacters)},
  physical_attack: ${physicalAttack},
  magical_attack: ${magicalAttack},
  meta: StandardItem
}); `

      console.log(out)
    })
  })
})

// Additional Work
// 1. sort all items as json objects by ItemType
// 2. write a function that processes each group and writes it to a file by group name to its own rs file
// 3. move this code into a crate instead that hosts all the game_data so other people can use it

        // m.insert( "AdventurersVest", Item {
        //     guid: ${guid},
        //     item_type: ItemType::None,
        //     name: ${name},
        //     order_priority: ${orderPriority},
        //     buy_price: ${buyPrice},
        //     sell_price: ${sellPrice},
        //     equippable_by: None
        //     meta: EquippableItem { 
        //         phy_def: 5,
        //         mag_def: 5,
        //         equippable_by:Some(&[
        //             PlayerPartyCharacter::Zale,
        //             PlayerPartyCharacter::Valere,
        //             PlayerPartyCharacter::Garl,
        //             PlayerPartyCharacter::Serai,
        //         ]),
        //         .. EquippableItem::default()
        //     },
        // });


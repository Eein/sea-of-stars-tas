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

  let out = `&[
`
  characters.forEach((c) => {
    out += `    ${c},
`
  })
  out += "  ]"

  return `Some(${out})`
}

const parseItemType = (type) => {
  switch(type) {
  case 'WEAPON': 
     return "ItemType::Weapon"
  case 'GROUP': 
     return "ItemType::GroupTrinket"
  case 'SNACK': 
     return "ItemType::Snack"
  case 'ARMOR': 
     return "ItemType::Armor"
  case 'RECIPEUNLOCK': 
     return "ItemType::Recipe"
  case 'TRINKET': 
     return "ItemType::Trinket"
  case 'CONCHITEM': 
     return "ItemType::Key"
  case 'KEYITEM': 
     return "ItemType::Key"
  case 'RELIC': 
     return "ItemType::Relic"
  case 'UNKNOWN': 
     return "ItemType::Unknown"
  case 'VALUABLEITEM': 
     return "ItemType::Valuable"
  case 'STORYARTIFACT': 
     return "ItemType::StoryArtifact"
  case 'INGREDIENT': 
     return "ItemType::Ingredient"
  case 'Armor': 
     return "ItemType::Armor"
  
  default:
    console.error(`Not type found for: ${type}`)
  }
}

let output = ""

// Reset the current output file
fs.writeFileSync('./output/output.rs', output)

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
      let physicalDefense = json?.physicalDefense || 0
      let magicalDefense = json?.magicalDefense || 0
      let hpToRestore = json?.hpToRestore || 0
      let mpToRestore = json?.mpToRestore || 0
      let hpUsePercent = json?.restoreHPPercentage || 0
      let mpUsePercent = json?.restoreMPPercentage || 0
      let hpPercentToRestore = json?.hpPercentageToRestore || 0.0
      let mpPercentToRestore = json?.mpPercentageToRestore || 0.0
      // Temporary = pull in localization file
      let localeKey = json?.nameLocalizationId?.locId
      let name = getFromLocale(json?.nameLocalizationId?.locId)
      let itemType = localeKey?.split("_")[0]

      // Overrides
      if(itemType == "DEFAULTCHESTCUTSCENE"){
        name = "INTERNAL[DEFAULT CHEST CUTSCENE]"
        itemType = "UNKNOWN"
      }
      if(guid == "a31ee5ffc1b693148be7c48150ebff81"){
        name = "Basic Armor"
      }

      let out = `// ${localeKey}
m.insert("${name}", Item {
  guid: "${guid}",
  item_type: ${parseItemType(itemType)},
  name: "${name}",
  order_priority: ${orderPriority},
  max_quantity: ${maxQuantity},
  buy_price: ${buyPrice},
  sell_price: ${sellPrice},
  equippable_by: ${formatEquippableBy(validCharacters)},
  physical_attack: ${physicalAttack},
  magical_attack: ${magicalAttack},
  physical_defense: ${physicalDefense},
  magical_defense: ${magicalDefense},
  hp_to_restore: ${hpToRestore},
  mp_to_restore: ${mpToRestore},
  // Restore mode
  hp_use_percent: ${hpUsePercent},
  mp_use_percent: ${mpUsePercent},
  // 0.0-1.0
  hp_percent_to_restore: ${hpPercentToRestore.toFixed(1)},
  mp_percent_to_restore: ${mpPercentToRestore.toFixed(1)},
});

m.insert("${guid}", Item {
  guid: "${guid}",
  item_type: ${parseItemType(itemType)},
  name: "${name}",
  order_priority: ${orderPriority},
  max_quantity: ${maxQuantity},
  buy_price: ${buyPrice},
  sell_price: ${sellPrice},
  equippable_by: ${formatEquippableBy(validCharacters)},
  physical_attack: ${physicalAttack},
  magical_attack: ${magicalAttack},
  physical_defense: ${physicalDefense},
  magical_defense: ${magicalDefense},
  hp_to_restore: ${hpToRestore},
  mp_to_restore: ${mpToRestore},
  // Restore mode
  hp_use_percent: ${hpUsePercent},
  mp_use_percent: ${mpUsePercent},
  // 0.0-1.0
  hp_percent_to_restore: ${hpPercentToRestore.toFixed(1)},
  mp_percent_to_restore: ${mpPercentToRestore.toFixed(1)},
});

`

        fs.writeFile('./output/output.rs', out, {flag: 'a' }, err => {
          if (err) {
            console.error(err);
          } else {
            // file written successfully
          }
        })
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


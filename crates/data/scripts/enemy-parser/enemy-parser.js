const fs = require('node:fs')
const output_file_path = './output/output.rs'

const enemy_locale_kv = require('./locale/Enemies-CAB-18cc0a97ed3fba0abaeabffc8cbc6c92-8020956684977417460.json')
const enemy_locale_definition = require('./locale/Enemies_EN-CAB-18cc0a97ed3fba0abaeabffc8cbc6c92--9219550977792058189.json')
const character_locale_kv = require('./locale/Characters-CAB-18cc0a97ed3fba0abaeabffc8cbc6c92--592314051786654548.json')
const character_locale_definition = require('./locale/Characters_EN-CAB-18cc0a97ed3fba0abaeabffc8cbc6c92--5266921922541985654.json')

let enemy_pairs = enemy_locale_kv.locIndexByLocStringId.keys.Array.map((val, i) => {
  return [val, enemy_locale_kv.locIndexByLocStringId.values.Array[i]]
})

let character_pairs = character_locale_kv.locIndexByLocStringId.keys.Array.map((val, i) => {
  return [val, character_locale_kv.locIndexByLocStringId.values.Array[i]]
})

const enemy_locale = Object.fromEntries(enemy_pairs)
const character_locale = Object.fromEntries(character_pairs)

const getFromLocale = (name) => {
  let eidx = enemy_locale[name]
  let enemy = enemy_locale_definition.strings.Array[eidx]

  let cidx = character_locale[name]
  let character = character_locale_definition.strings.Array[cidx] 
  return enemy || character
}

let output = ""
let names = []

// Reset the current output file
fs.writeFileSync(output_file_path, output)

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
        // console.log("SKIPPING GUID UNDEFINED")
        return
      }
      let physicalAttack = json?.basePhysicalAttack || 0
      let magicalAttack = json?.baseMagicalAttack || 0
      let physicalDefense = json?.basePhysicalDefense || 0
      let magicalDefense = json?.baseMagicalDefense || 0
      let hp = json?.hp || 0
      let speed = json?.speed || 0
      let liveManaSpawnQuantity = json?.liveManaSpawnQuantity || 0
      let enemyLevel = json?.enemyLevel || 0
      let fleshmancerMinion = json?.fleshmancerMinion || 0
      let damageTypeModifiers = json?.damageTypeModifiers || { keys: [], values: [] }

      // TODO:
      // qualifiers
      // damage type modifiers



      // Temporary = pull in localization file
      let localeKey = json?.nameLocalizationId?.locId
      let name = getFromLocale(json?.nameLocalizationId?.locId.toUpperCase())

      // Overrides
      // These enemies are either
      // - repeat enemies (ex: Brugaves/Erlina)
      // - Arena enemies
      // - ng+ maybe?
      // - improved bosses (ex: sea slug)
      // They share the same names in some cases, but to differentiate,
      // I'm just doing `B`/`C`/etc for now.
      switch (guid) {
        case 'e1685476dd793e44c9c8909fe0b3622f':
          name = "Dweller Of Woe Banshee"
          break;
        case 'bcde1eb0ea076f846a0ee20287d88204':
          name = "Dweller Of Woe Totem"
          break;
        case 'ae0a040c2cb657d4b9d10aaa9603ac15':
          name = "Mangler Fish B"
          break;
        case '6360b75144ae73845980a1c1e25ccabb':
          name = "Elder Mist's Arm B"
          break;
        case '7b27481afea42bb479dc3f81032a16d9':
          name = "Elder Mist's Sword B"
          break;
        case '6b3a54899475c384385fdcee4fa1fe26':
          name = "Elder Mist B"
          break;
        case '3259041d34de847448e496200e994a8a':
          name = "Measure Chest B"
          break;
        case 'c5fc39dac699f0645a0fa83505b12b2a':
          name = "Shrimp Knight B"
          break;
        case 'd29103a8ea0fec54fac2a092acf1308a':
          name = "Sleuth B"
          break;
        case '4578c77a3346d0845bae8eb8f4bb1cd1':
          name = "BilePile B"
          break;
        case 'a5a0da6d66767df47b2523b04f14738a':
          name = "Mermofwizquard B"
          break;
        case '987f0860e9c1f5a43997e14a219dbe91':
          name = "Mermofwizquard C"
          break;
        case '1894c41627be94d408bd64295ab6dd18':
          name = "Erlina B"
          break;
        case 'f71d669f95fdec7498d0ec4e4bab0e81':
          name = "Erlina C"
          break;
        case 'cc767e360aab54d4ca314a206e32ffee':
          name = "Brugraves B"
          break;
        case '96a30eb1f53ec1c4ebe90da25545b1ac':
          name = "Brugraves C"
          break;
        case '54abc79fbf9dd2f4a8bd19cab8245391':
          name = "Phase Reaper B"
          break;
        case 'f0a9344cdf22e654c9e3c6ef7b1508d6':
          name = "Zale B"
          break;
        case 'f802ea770f9b9da4c8d95ccb485a79d9':
          name = "Seraï B"
          break;
        case '17869a064ace26541bf54688ee3f7f93':
          name = "Tsiclop B"
          break;
        case '3ac5141f57d77c642948246353f8b5b4':
          name = "Valere B"
          break;
        case '42c3cf27ee18164428ad318882f5137e':
          name = "Wanderer B"
          break;
        case 'd7cdfe62090e94047991a1b9ca612a6d':
          name = "Lonzon B"
          break;
        case '1e62e41d77ce0f344bb884ffb4d92ebf':
          name = "Lonzon C"
          break;
        case '2167ae398eb2ca3409431e9c3e48e7bc':
          name = "Lonzon D"
          break;
        case '9e34c0f2e5678124f8e503389bd174be':
          name = "Gulgul B"
          break;
        case 'cac4ad5c6feebe443bee7570b8e009b8':
          name = "Gulgul C"
          break;
        case '4608a56ce03fa8f42a467a917c438bcc':
          name = "Big Buggy B"
          break;
        case 'd5f7b3f22f54712468481c6a987ddb85':
          name = "Sea Bossbug"
          break;
        case '5064bcbd33aceb1418e0ef6b4ed40515':
          name = "Owlsassin B"
          break;
        case '0e5b91e5ad0b2784da76ba6314004370':
          name = "Elysan'darëlle B"
          break;
        case 'ca5e21141b95aad49bc08403f495b38d':
          name = "Srower B"
          break;
        case '527885ffe2b65d049b17ebbe2d19136d':
          name = "Croube B"
          break;
        case 'e92dfcf5e74aeb34f98f32b8cd563ebf':
          name = "Croube C"
          break;
        case '14c5d91be67e5214ba8ce66c21a282e7':
          name = "Croube D"
          break;
        case 'e126c542eee0aa7468e3f62ad953ca4d':
          name = "Training Croube B"
          break;
        case '2ef49b6d9ec8fd64f95c49e951c1c8bc':
          name = "Sling Rabbit B"
          break;
        case '9b7d5ffcd415b664abdc0987f7776524':
          name = "Scout B"
          break;
        case 'e0e39853cf0aedc4a87abc25605ea4a6':
          name = "GooGoon B"
          break;
        case 'd788814517a8e1549b0253e534126938':
          name = "Rochèvre B"
          break;
        case 'ea0a539bb73e45a42ae867fce0822b92':
          name = "Garnooy B"
          break;
        case '6c2a1e2872a0b5a469d1f9e437f60fb8':
          name = "Shroomy Shroomy Knight B"
          break;
        case 'ce1df87d1facdf2469716190f2d6ad51':
          name = "Romaya B"
          break;
        case '09ab35a6a52c0c74f836febf7d6e7a2e':
          name = "Anointed B"
          break;
        case 'e6ac627711e4ee44da103c47d1cd5736':
          name = "Ant Bruiser B"
          break;
        case 'd0a6e3e6b8288bf4bb7abd1cedb38ca3':
          name = "Bone Pile B"
          break;
        case 'a43bbafa0d9b4c54a82079c03c9b638e':
          name = "Flesh Pile B"
          break;
        case '470302d28c08352438d633aebb7c0cb5':
          name = "Grassassin B"
          break;
        case '980a2ead2b197f947aa5199927376dbb':
          name = "Melee Matey B"
          break;
        case 'b10c6cc7a49f77246848a74cae5ea119':
          name = "Ranged Matey B"
          break;
        case 'eef757b01f5cd80459e634f109a1e69c':
          name = "Revenant B"
          break;
        case '4d5fce1b06b687c4fbf91daa5b2b1d67':
          name = "Revenant C"
          break;
        case '5e1fb9276fd3e714d8fe1a4cfa8681af':
          name = "Woodland Spirit B"
          break;

        default:
      }
      if(name == undefined || name == 'undefined'){
        name = localeKey
        if(!name){
          let mName = json?.m_Name;
          switch (mName) {
            case 'BabyPraPraData':
              name = "Baby Pra Pra"
              break;
            case 'CrystalPrisonData':
              name = "Crystal Prison"
              break;
            case 'GenericEnemyData':
              name = "Generic Enemy"
              break;
            case 'SylgainCannonData':
              name = "Sylgain Cannon"
              break;
            default:

          }
        }
        
      }

      const parseDamageTypeModifiers = (modifiers) => {

        output = ''
        // console.log('len', modifiers?.keys.length, modifiers)
        if(modifiers?.keys.length === 0){
          return 'None'
          // or Some(&[])
        }
        output += `Some(&[
        `

        let damage_modifier_pairs = modifiers.keys.map((val, i) => {
          return [val, modifiers.values[i]]
        })
        // console.log(damage_modifier_pairs)


        damage_modifier_pairs.forEach((mod) => {
          type = "DamageType::None"
          switch (mod[0]) {
            case 0:
              type = "CombatDamageType::None"
              break
            case 1:
              type = "CombatDamageType::Any"
              break
            case 2:
              type = "CombatDamageType::Sword"
              break
            case 4:
              type = "CombatDamageType::Sun"
              break
            case 8:
              type = "CombatDamageType::Moon"
              break
            case 16:
              type = "CombatDamageType::Eclipse"
              break
            case 32:
              type = "CombatDamageType::Poison"
              break
            case 64:
              type = "CombatDamageType::Arcane"
              break
            case 128:
              type = "CombatDamageType::Stun"
              break
            case 256:
              type = "CombatDamageType::Blunt"
              break
            case 252:
              type = "CombatDamageType::Magical"
              break
            default: 
              type = "CombatDamageType::None"
          }

          output += `DamageTypeModifier { type: ${type}, modifier: ${Number.parseFloat(mod[1])} },
        `
        })

        output += `])`
        // console.log(output)
        return output
      }

      let dtmods = parseDamageTypeModifiers(damageTypeModifiers)
      
      names.push(name)
      // console.log(name)
      
      let out = `// ${localeKey}
m.insert("${name}", Enemy {
  guid: "${guid}",
  name: "${name}",
  hp: ${hp},
  speed: ${speed},
  damage_type_modifiers: ${dtmods},
  live_mana_spawn_quantity: ${liveManaSpawnQuantity},
  level: ${enemyLevel},
  fleshmancer_minion: ${fleshmancerMinion},
  physical_attack: ${physicalAttack},
  magical_attack: ${magicalAttack},
  physical_defense: ${physicalDefense},
  magical_defense: ${magicalDefense},
});

m.insert("${guid}", Enemy {
  guid: "${guid}",
  name: "${name}",
  hp: ${hp},
  speed: ${speed},
  damage_type_modifiers: ${dtmods},
  live_mana_spawn_quantity: ${liveManaSpawnQuantity},
  level: ${enemyLevel},
  fleshmancer_minion: ${fleshmancerMinion},
  physical_attack: ${physicalAttack},
  magical_attack: ${magicalAttack},
  physical_defense: ${physicalDefense},
  magical_defense: ${magicalDefense},
});

`

        fs.writeFile(output_file_path, out, {flag: 'a' }, err => {
          if (err) {
            console.error(err);
          } else {
            // file written successfully
          }
        })
      // console.log(names.sort().filter((e, i, a) => a.indexOf(e) !== i))
    })
  })

})


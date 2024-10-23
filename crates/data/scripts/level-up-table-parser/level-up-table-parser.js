const fs = require('node:fs')

const valere_table = require('./input/ValereLevelUpTable-CAB-5db72f930a39b8179e97cebb848bbc2a-7558692442236353842.json')
const zale_table = require('./input/ZaleLevelUpTable-CAB-5db72f930a39b8179e97cebb848bbc2a-2906195274032018014.json')
const bst_table = require('./input/BstLevelUpTable-CAB-5db72f930a39b8179e97cebb848bbc2a--8190115954139494845.json')
const garl_table = require('./input/GarlLevelUpTable-CAB-5db72f930a39b8179e97cebb848bbc2a--8108360792447049677.json')
const serai_table = require('./input/SeraiLevelUpTable-CAB-5db72f930a39b8179e97cebb848bbc2a-7458494238719453766.json')
const reshan_table = require('./input/ReshanLevelUpTable-CAB-5db72f930a39b8179e97cebb848bbc2a--7868004120263399585.json')

let tables = [
  valere_table,
  zale_table,
  bst_table,
  garl_table,
  serai_table,
  reshan_table
]

const camel_to_snake = str => str[0].toLowerCase() + str.slice(1, str.length).replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);

let output = ""
let output_sum = ""

// Reset the current output file

tables.forEach((table) => {
  let name = camel_to_snake(table.m_Name)
  console.log(name)
  // let summed_name = camel_to_snake(table.m_Name) + `_sum`
  fs.writeFileSync(`./output/${name}.rs`, output)

  table.levelUpStats.Array.forEach((stat, index) =>  {
     output += `// Level ${index+2} - ${name}
m.insert(${index+2}, LevelUpStats { 
  hp: ${stat.hp}, 
  mp: ${stat.sp},
  physical_attack: ${stat.pa},
  physical_defense: ${stat.pd},
  magical_attack: ${stat.ma},
  magical_defense: ${stat.md},
});

`

  });
  // 
  fs.writeFileSync(`./output/${name}.rs`, output, {flag: 'a' }, err => {
    if (err) {
      console.error(err);
    } else {
      // file written successfully
    }
  })
  output = ""
})


tables.forEach((table) => {
  let name = camel_to_snake(table.m_Name) + "_sum"
  console.log(name)
  fs.writeFileSync(`./output/${name}.rs`, output)

  let hp = 0
  let sp = 0
  let pa = 0
  let pd = 0
  let ma = 0
  let md = 0

  table.levelUpStats.Array.forEach((stat, index) =>  {
    hp += stat.hp
    sp += stat.sp
    pa += stat.pa
    pd += stat.pd
    ma += stat.ma
    md += stat.md

     output_sum += `// Sum Level ${index+2} - ${name}
m.insert(${index+2}, LevelUpStats { 
  hp: ${hp}, 
  mp: ${sp},
  physical_attack: ${pa},
  physical_defense: ${pd},
  magical_attack: ${ma},
  magical_defense: ${md},
});

`

  });
  // 
  fs.writeFileSync(`./output/${name}.rs`, output_sum, {flag: 'a' }, err => {
    if (err) {
      console.error(err);
    } else {
      // file written successfully
    }
  })

  hp = 0
  sp = 0
  pa = 0
  ma = 0
  md = 0
  output_sum = ""
})

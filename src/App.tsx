import { useState } from "react";
import { Button, Flex, Grid, Heading, TextField, View } from "@adobe/react-spectrum";
import { EffectPropertyType, commands } from "./bindings.gen";
import { SideNav, SideNavButton } from "./components/SideNav";
import Beaker from "@spectrum-icons/workflow/Beaker";
import PeopleGroup from "@spectrum-icons/workflow/PeopleGroup";
import { MultiComboBox } from "./components/MultiComboBox";

function App() {
  const [greetMsg, setGreetMsg] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // setGreetMsg(await invoke("greet", { name }));
    setGreetMsg(await commands.prankHimJohn(
      {
        Jingliu: {
          e1_crit_dmg: true,
          e2_skill_buff: true,
          enhanced_state: true,
          hp_drain_pct: 1.0
        }
      },
      {
        level: 80,
        eidolon: 6,
        ascension: 6,
        skills: {
          basic: 7 - 1,
          skill: 12 - 1,
          ult: 12 - 1,
          talent: 12 - 1,
        },
        traces: {
          ability_1: true,
          ability_2: true,
          ability_3: true,
          stat_1: true,
          stat_2: true,
          stat_3: true,
          stat_4: true,
          stat_5: true,
          stat_6: true,
          stat_7: true,
          stat_8: true,
          stat_9: true,
          stat_10: true,
        }
      },
      {
        IShallBeMyOwnSword: {
          eclipse_stacks: 3,
        }
      },
      {
        ascension: 6,
        level: 80,
        superimposition: 5 - 1
      },
      {
        count: 1,
        level: 95,
    
        resistance: 0.2,
        elemental_weakness: true,
        weakness_broken: false,
      }
    ))
  }

  return (
    <View
      minHeight={"100vh"}
      // padding={"size-200"}
      // UNSAFE_style={{ boxSizing: "border-box" }}
    >
      <Grid
        areas={["header header", "sidebar content"]}
        columns={["size-2400", "auto"]}
        rows={["size-500", "auto"]}
        height={"100vh"}
        gap={"size-200"}
      >
        <View 
          gridArea="sidebar"
          padding={"size-100"}
        >
          {/* <ListBox>
            
          </ListBox> */}
          <SideNav default="optimizer">
            <SideNavButton id="optimizer"><Beaker size="S"/> Optimizer</SideNavButton>
            <SideNavButton id="characters"><PeopleGroup size="S"/> Characters</SideNavButton>
          </SideNav>
        </View>
        <View gridArea="header">
          <Flex alignItems="end" height="100%" marginStart="size-200">
            <Heading level={2}>Free Honkai Star Rrail Cheat 100% Working Infinite Stellar Jade 2023</Heading>
          </Flex>
        </View>
        <View gridArea="content">
          <Flex alignItems="end" gap="size-200">
            <Button variant="cta" onPress={() => greet()}>Greet</Button>
          </Flex>
          <p>{greetMsg}</p>
          <MultiComboBox<EffectPropertyType> label="Chest Filter" options={[
            {label: "CRIT Rate", value: "CriticalChanceBase"},
            {label: "CRIT DMG", value: "CriticalDamageBase"},
            {label: "ATK%", value: "AttackAddedRatio"},
            {label: "DEF%", value: "DefenceAddedRatio"},
            {label: "HP%", value: "HPAddedRatio"},
            {label: "Effect Hit Rate", value: "StatusProbabilityBase"},
            {label: "Outgoing Healing Boost", value: "HealRatioBase"},
          ]}/>
          <MultiComboBox<EffectPropertyType> label="Feet Pics" options={[
            {label: "ATK%", value: "AttackAddedRatio"},
            {label: "DEF%", value: "DefenceAddedRatio"},
            {label: "HP%", value: "HPAddedRatio"},
            {label: "SPD", value: "SpeedDelta"},
          ]}/>
          <MultiComboBox<EffectPropertyType> label="Planar Sphere Filter" options={[
            {label: "ATK%", value: "AttackAddedRatio"},
            {label: "DEF%", value: "DefenceAddedRatio"},
            {label: "HP%", value: "HPAddedRatio"},
            {label: "Physical DMG Boost", value: "PhysicalAddedRatio"},
            {label: "Fire DMG Boost", value: "FireAddedRatio"},
            {label: "Ice DMG Boost", value: "IceAddedRatio"},
            {label: "Thunder DMG Boost", value: "ThunderAddedRatio"},
            {label: "Wind DMG Boost", value: "WindAddedRatio"},
            {label: "Quantum DMG Boost", value: "QuantumAddedRatio"},
            {label: "Imaginary DMG Boost", value: "ImaginaryAddedRatio"},
          ]}/>
          <MultiComboBox<EffectPropertyType> label="Link Rope Filter" options={[
            {label: "ATK%", value: "AttackAddedRatio"},
            {label: "DEF%", value: "DefenceAddedRatio"},
            {label: "HP%", value: "HPAddedRatio"},
            {label: "Break Effect", value: "BreakDamageAddedRatioBase"},
            {label: "Energy Regeneration Rate", value: "SPRatioBase"},
          ]}/>
        </View>
      </Grid>
      

      {/* <Tabs orientation="vertical">
        <TabList>
          <Item key="a">Tab 1</Item>
          <Item key="b">Tab 2</Item>
          <Item key="c">Tab 3</Item>
          <Item key="d">Tab 4</Item>
          <Item key="e">Tab 5</Item>
        </TabList>
        <TabPanels>
          <Item key="a">
            <p>Tab 1</p>
          </Item>
          <Item key="b">
            <p>Tab 2</p>
          </Item>
          <Item key="c">
            <p>Tab 3</p>
          </Item>
          <Item key="d">
            <p>Tab 4</p>
          </Item>
          <Item key="e">
            <p>Tab 5</p>
          </Item>
        </TabPanels>
      </Tabs> */}
      
    </View>
  );
}

export default App;

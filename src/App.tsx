import { useState } from "react";
import { Button, Flex, Grid, Heading, TextField, View } from "@adobe/react-spectrum";
import { commands } from "./bindings.gen";
import { SideNav, SideNavButton } from "./components/SideNav";
import Beaker from "@spectrum-icons/workflow/Beaker";
import PeopleGroup from "@spectrum-icons/workflow/PeopleGroup";
import { MultiComboBox } from "./components/MultiComboBox";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // setGreetMsg(await invoke("greet", { name }));
    setGreetMsg(await commands.greet(name))
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
            <TextField 
              label="Name"
              onChange={setName}
            />
            <Button variant="cta" onPress={() => greet()}>Greet</Button>
          </Flex>
          <p>{greetMsg}</p>
          <MultiComboBox label="Test" />
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

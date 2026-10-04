import React from "react";
import { Sidebar } from "./components/Sidebar";
import { Inbox } from "./components/Inbox";
import { MessageView } from "./components/MessageView";
import { DashboardView } from "./components/DashboardViews";
import { demoMessages } from "./mock";
import type { NavKey } from "./types";

export default function App() {
  const [nav,setNav]=React.useState<NavKey>("inbox");
  const [selected,setSelected]=React.useState(demoMessages[1]?.id ?? "");
  const [query,setQuery]=React.useState("");
  const current=demoMessages.find(m=>m.id===selected);
  return <div className="appShell">
    <Sidebar active={nav} onChange={setNav}/>
    {nav==="inbox" ? <main className="mailWorkspace"><Inbox messages={demoMessages} selected={selected} onSelect={setSelected} query={query} onQuery={setQuery}/><MessageView message={current}/></main> : <main className="singleWorkspace"><DashboardView view={nav} messages={demoMessages}/></main>}
  </div>;
}

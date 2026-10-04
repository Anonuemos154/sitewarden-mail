import type { NavKey } from "../types";

const items: Array<[NavKey,string,string]> = [
  ["inbox","Inbox","04"], ["security","Security","02"], ["cleanup","Cleanup","18"],
  ["quarantine","Quarantine","01"], ["rules","Rules",""], ["productivity","Productivity",""],
  ["accounts","Accounts","02"], ["settings","Settings",""]
];

export function Sidebar({active,onChange}:{active:NavKey,onChange:(v:NavKey)=>void}) {
  return <aside className="sidebar">
    <div className="brand"><span className="brandMark">SW</span><div><strong>SiteWarden Mail</strong><small>by MSD Systems Lab</small></div></div>
    <div className="alphaBadge">ALPHA · LOCAL DEMO</div>
    <nav>{items.map(([key,label,count]) => <button className={active===key?"nav active":"nav"} key={key} onClick={()=>onChange(key)}><span>{label}</span><em>{count}</em></button>)}</nav>
    <div className="sidebarFoot"><span className="statusDot"/>Security core: foundation<br/><small>No real mailbox connected</small></div>
  </aside>;
}

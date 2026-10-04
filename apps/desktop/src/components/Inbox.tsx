import type { MailItem } from "../types";

function RiskPill({score}:{score:number}) {
  const level=score>=70?"high":score>=30?"medium":"low";
  return <span className={`riskPill ${level}`}>{score}</span>;
}

export function Inbox({messages,selected,onSelect,query,onQuery}:{messages:MailItem[],selected:string,onSelect:(id:string)=>void,query:string,onQuery:(v:string)=>void}) {
  const filtered=messages.filter(m => `${m.sender} ${m.subject} ${m.preview}`.toLowerCase().includes(query.toLowerCase()));
  return <section className="inboxPane">
    <div className="paneHead"><div><p className="eyebrow">UNIFIED</p><h2>Inbox</h2></div><span>{filtered.length} messages</span></div>
    <div className="search"><span>⌕</span><input value={query} onChange={e=>onQuery(e.target.value)} placeholder="Search local index"/></div>
    <div className="messageList">{filtered.map(m=><button className={`messageRow ${m.id===selected?"selected":""}`} key={m.id} onClick={()=>onSelect(m.id)}>
      <span className={m.unread?"unreadDot on":"unreadDot"}/><div className="messageMain"><div className="messageMeta"><strong>{m.sender}</strong><time>{m.received}</time></div><div className="subject">{m.subject}</div><div className="preview">{m.preview}</div><div className="tags"><span>{m.account}</span>{m.quarantined&&<span className="dangerText">QUARANTINED</span>}</div></div><RiskPill score={m.riskScore}/>
    </button>)}</div>
  </section>;
}

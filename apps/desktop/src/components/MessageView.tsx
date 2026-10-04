import type { MailItem } from "../types";

export function MessageView({message}:{message?:MailItem}) {
  if(!message) return <section className="messagePane empty">Select a message.</section>;
  return <section className="messagePane">
    <div className="messageToolbar"><span className="safeTag">SAFE VIEW</span><div><button>Archive</button><button>Trash</button><button className="primary">Reply</button></div></div>
    <header className="mailHeader"><p className="eyebrow">{message.senderAddress}</p><h1>{message.subject}</h1><p>{message.sender} · {message.account}</p></header>
    {message.signals.length>0 && <div className="signalStack">{message.signals.map(s=><article className={`signal ${s.severity}`} key={s.code}><div><span>{s.severity.toUpperCase()}</span><strong>{s.title}</strong></div><p>{s.explanation}</p><code>{s.code}</code></article>)}</div>}
    <article className="safeBody"><pre>{message.safeText}</pre></article>
    {message.links.length>0 && <section className="linkInspector"><h3>Link Inspector</h3>{message.links.map((l,i)=><div className="linkCard" key={i}><strong>{l.label}</strong><code>{l.destination}</code>{l.warning&&<span>{l.warning}</span>}</div>)}</section>}
    <footer className="trustFoot">Original message HTML is not rendered in this demonstration view.</footer>
  </section>;
}

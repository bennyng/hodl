function t(){}function n(t,n){for(const e in n)t[e]=n[e];return t}function e(t){return t()}function o(){return Object.create(null)}function r(t){t.forEach(e)}function c(t){return"function"==typeof t}function s(t,n){return t!=t?n==n:t!==n||t&&"object"==typeof t||"function"==typeof t}function u(t,n,e,o){if(t){const r=i(t,n,e,o);return t[0](r)}}function i(t,e,o,r){return t[1]&&r?n(o.ctx.slice(),t[1](r(e))):o.ctx}function f(t,n,e,o,r,c,s){const u=function(t,n,e,o){if(t[2]&&o){const r=t[2](o(e));if(void 0===n.dirty)return r;if("object"==typeof r){const t=[],e=Math.max(n.dirty.length,r.length);for(let o=0;o<e;o+=1)t[o]=n.dirty[o]|r[o];return t}return n.dirty|r}return n.dirty}(n,o,r,c);if(u){const r=i(n,e,o,s);t.p(r,u)}}function a(t,n){t.appendChild(n)}function l(t,n,e){t.insertBefore(n,e||null)}function d(t){t.parentNode.removeChild(t)}function h(t){return document.createElement(t)}function p(t){return document.createTextNode(t)}function g(){return p(" ")}function $(){return p("")}function m(t,n,e){null==e?t.removeAttribute(n):t.getAttribute(n)!==e&&t.setAttribute(n,e)}function y(t){return Array.from(t.childNodes)}function b(t,n,e,o){for(let r=0;r<t.length;r+=1){const o=t[r];if(o.nodeName===n){let n=0;const c=[];for(;n<o.attributes.length;){const t=o.attributes[n++];e[t.name]||c.push(t.name)}for(let t=0;t<c.length;t++)o.removeAttribute(c[t]);return t.splice(r,1)[0]}}return o?function(t){return document.createElementNS("http://www.w3.org/2000/svg",t)}(n):h(n)}function x(t,n){for(let e=0;e<t.length;e+=1){const o=t[e];if(3===o.nodeType)return o.data=""+n,t.splice(e,1)[0]}return p(n)}function _(t){return x(t," ")}function w(t,n){n=""+n,t.wholeText!==n&&(t.data=n)}let k;function v(t){k=t}function E(){if(!k)throw new Error("Function called outside component initialization");return k}function j(t){E().$$.on_mount.push(t)}function A(t){E().$$.after_update.push(t)}function S(t,n){E().$$.context.set(t,n)}const M=[],N=[],O=[],C=[],T=Promise.resolve();let q=!1;function z(t){O.push(t)}let B=!1;const F=new Set;function D(){if(!B){B=!0;do{for(let t=0;t<M.length;t+=1){const n=M[t];v(n),G(n.$$)}for(v(null),M.length=0;N.length;)N.pop()();for(let t=0;t<O.length;t+=1){const n=O[t];F.has(n)||(F.add(n),n())}O.length=0}while(M.length);for(;C.length;)C.pop()();q=!1,B=!1,F.clear()}}function G(t){if(null!==t.fragment){t.update(),r(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(z)}}const H=new Set;let I;function P(){I={r:0,c:[],p:I}}function J(){I.r||r(I.c),I=I.p}function K(t,n){t&&t.i&&(H.delete(t),t.i(n))}function L(t,n,e,o){if(t&&t.o){if(H.has(t))return;H.add(t),I.c.push((()=>{H.delete(t),o&&(e&&t.d(1),o())})),t.o(n)}}function Q(t,n){t.d(1),n.delete(t.key)}function R(t,n,e,o,r,c,s,u,i,f,a,l){let d=t.length,h=c.length,p=d;const g={};for(;p--;)g[t[p].key]=p;const $=[],m=new Map,y=new Map;for(p=h;p--;){const t=l(r,c,p),u=e(t);let i=s.get(u);i?o&&i.p(t,n):(i=f(u,t),i.c()),m.set(u,$[p]=i),u in g&&y.set(u,Math.abs(p-g[u]))}const b=new Set,x=new Set;function _(t){K(t,1),t.m(u,a),s.set(t.key,t),a=t.first,h--}for(;d&&h;){const n=$[h-1],e=t[d-1],o=n.key,r=e.key;n===e?(a=n.first,d--,h--):m.has(r)?!s.has(o)||b.has(o)?_(n):x.has(r)?d--:y.get(o)>y.get(r)?(x.add(o),_(n)):(b.add(r),d--):(i(e,s),d--)}for(;d--;){const n=t[d];m.has(n.key)||i(n,s)}for(;h;)_($[h-1]);return $}function U(t,n){const e={},o={},r={$$scope:1};let c=t.length;for(;c--;){const s=t[c],u=n[c];if(u){for(const t in s)t in u||(o[t]=1);for(const t in u)r[t]||(e[t]=u[t],r[t]=1);t[c]=u}else for(const t in s)r[t]=1}for(const s in o)s in e||(e[s]=void 0);return e}function V(t){return"object"==typeof t&&null!==t?t:{}}function W(t){t&&t.c()}function X(t,n){t&&t.l(n)}function Y(t,n,o,s){const{fragment:u,on_mount:i,on_destroy:f,after_update:a}=t.$$;u&&u.m(n,o),s||z((()=>{const n=i.map(e).filter(c);f?f.push(...n):r(n),t.$$.on_mount=[]})),a.forEach(z)}function Z(t,n){const e=t.$$;null!==e.fragment&&(r(e.on_destroy),e.fragment&&e.fragment.d(n),e.on_destroy=e.fragment=null,e.ctx=[])}function tt(t,n){-1===t.$$.dirty[0]&&(M.push(t),q||(q=!0,T.then(D)),t.$$.dirty.fill(0)),t.$$.dirty[n/31|0]|=1<<n%31}function nt(n,e,c,s,u,i,f=[-1]){const a=k;v(n);const l=n.$$={fragment:null,ctx:null,props:i,update:t,not_equal:u,bound:o(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(a?a.$$.context:e.context||[]),callbacks:o(),dirty:f,skip_bound:!1};let h=!1;if(l.ctx=c?c(n,e.props||{},((t,e,...o)=>{const r=o.length?o[0]:e;return l.ctx&&u(l.ctx[t],l.ctx[t]=r)&&(!l.skip_bound&&l.bound[t]&&l.bound[t](r),h&&tt(n,t)),e})):[],l.update(),h=!0,r(l.before_update),l.fragment=!!s&&s(l.ctx),e.target){if(e.hydrate){const t=y(e.target);l.fragment&&l.fragment.l(t),t.forEach(d)}else l.fragment&&l.fragment.c();e.intro&&K(n.$$.fragment),Y(n,e.target,e.anchor,e.customElement),D()}v(a)}class et{$destroy(){Z(this,1),this.$destroy=t}$on(t,n){const e=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return e.push(n),()=>{const t=e.indexOf(n);-1!==t&&e.splice(t,1)}}$set(t){var n;this.$$set&&(n=t,0!==Object.keys(n).length)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}const ot=[];function rt(n,e=t){let o;const r=[];function c(t){if(s(n,t)&&(n=t,o)){const t=!ot.length;for(let e=0;e<r.length;e+=1){const t=r[e];t[1](),ot.push(t,n)}if(t){for(let t=0;t<ot.length;t+=2)ot[t][0](ot[t+1]);ot.length=0}}}return{set:c,update:function(t){c(t(n))},subscribe:function(s,u=t){const i=[s,u];return r.push(i),1===r.length&&(o=e(c)||t),s(n),()=>{const t=r.indexOf(i);-1!==t&&r.splice(t,1),0===r.length&&(o(),o=null)}}}}export{n as A,P as B,rt as C,u as D,f as E,a as F,t as G,R as H,Q as I,et as S,y as a,m as b,b as c,d,h as e,l as f,x as g,w as h,nt as i,W as j,g as k,$ as l,X as m,_ as n,Y as o,U as p,V as q,L as r,s,p as t,J as u,K as v,Z as w,S as x,A as y,j as z};

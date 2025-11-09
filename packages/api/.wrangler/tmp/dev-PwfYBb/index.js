var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// .wrangler/tmp/bundle-5eje84/checked-fetch.js
var urls = /* @__PURE__ */ new Set();
function checkURL(request, init) {
  const url = request instanceof URL ? request : new URL(
    (typeof request === "string" ? new Request(request, init) : request).url
  );
  if (url.port && url.port !== "443" && url.protocol === "https:") {
    if (!urls.has(url.toString())) {
      urls.add(url.toString());
      console.warn(
        `WARNING: known issue with \`fetch()\` requests to custom HTTPS ports in published Workers:
 - ${url.toString()} - the custom port will be ignored when the Worker is published using the \`wrangler deploy\` command.
`
      );
    }
  }
}
__name(checkURL, "checkURL");
globalThis.fetch = new Proxy(globalThis.fetch, {
  apply(target, thisArg, argArray) {
    const [request, init] = argArray;
    checkURL(request, init);
    return Reflect.apply(target, thisArg, argArray);
  }
});

// build/index.js
import { WorkerEntrypoint as gt } from "cloudflare:workers";
import $ from "./1c683ab72cf2497338ebf9f0f7f9bb2c75067787-index_bg.wasm";
var i;
var A = null;
function P() {
  return (A === null || A.byteLength === 0) && (A = new Uint8Array(i.memory.buffer)), A;
}
__name(P, "P");
var V = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
V.decode();
function X(t, e) {
  return V.decode(P().subarray(t, t + e));
}
__name(X, "X");
function p(t, e) {
  return t = t >>> 0, X(t, e);
}
__name(p, "p");
var d = new Array(128).fill(void 0);
d.push(void 0, null, true, false);
var x = d.length;
function s(t) {
  x === d.length && d.push(d.length + 1);
  let e = x;
  return x = d[e], d[e] = t, e;
}
__name(s, "s");
function r(t) {
  return d[t];
}
__name(r, "r");
var l = 0;
var O = new TextEncoder();
"encodeInto" in O || (O.encodeInto = function(t, e) {
  let n = O.encode(t);
  return e.set(n), { read: t.length, written: n.length };
});
function k(t, e, n) {
  if (n === void 0) {
    let a = O.encode(t), h = e(a.length, 1) >>> 0;
    return P().subarray(h, h + a.length).set(a), l = a.length, h;
  }
  let _ = t.length, o = e(_, 1) >>> 0, w = P(), f = 0;
  for (; f < _; f++) {
    let a = t.charCodeAt(f);
    if (a > 127) break;
    w[o + f] = a;
  }
  if (f !== _) {
    f !== 0 && (t = t.slice(f)), o = n(o, _, _ = f + t.length * 3, 1) >>> 0;
    let a = P().subarray(o + f, o + _), h = O.encodeInto(t, a);
    f += h.written, o = n(o, _, f, 1) >>> 0;
  }
  return l = f, o;
}
__name(k, "k");
var m = null;
function u() {
  return (m === null || m.buffer.detached === true || m.buffer.detached === void 0 && m.buffer !== i.memory.buffer) && (m = new DataView(i.memory.buffer)), m;
}
__name(u, "u");
function b(t) {
  return t == null;
}
__name(b, "b");
function L(t) {
  let e = typeof t;
  if (e == "number" || e == "boolean" || t == null) return `${t}`;
  if (e == "string") return `"${t}"`;
  if (e == "symbol") {
    let o = t.description;
    return o == null ? "Symbol" : `Symbol(${o})`;
  }
  if (e == "function") {
    let o = t.name;
    return typeof o == "string" && o.length > 0 ? `Function(${o})` : "Function";
  }
  if (Array.isArray(t)) {
    let o = t.length, w = "[";
    o > 0 && (w += L(t[0]));
    for (let f = 1; f < o; f++) w += ", " + L(t[f]);
    return w += "]", w;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(t)), _;
  if (n && n.length > 1) _ = n[1];
  else return toString.call(t);
  if (_ == "Object") try {
    return "Object(" + JSON.stringify(t) + ")";
  } catch {
    return "Object";
  }
  return t instanceof Error ? `${t.name}: ${t.message}
${t.stack}` : _;
}
__name(L, "L");
function g(t, e) {
  try {
    return t.apply(this, e);
  } catch (n) {
    i.__wbindgen_export3(s(n));
  }
}
__name(g, "g");
function q(t, e) {
  return t = t >>> 0, P().subarray(t / 1, t / 1 + e);
}
__name(q, "q");
var N = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => {
  t.instance === c && t.dtor(t.a, t.b);
});
function Y(t, e, n, _) {
  let o = { a: t, b: e, cnt: 1, dtor: n, instance: c }, w = /* @__PURE__ */ __name((...f) => {
    if (o.instance !== c) throw new Error("Cannot invoke closure from previous WASM instance");
    o.cnt++;
    let a = o.a;
    o.a = 0;
    try {
      return _(a, o.b, ...f);
    } finally {
      o.a = a, w._wbg_cb_unref();
    }
  }, "w");
  return w._wbg_cb_unref = () => {
    --o.cnt === 0 && (o.dtor(o.a, o.b), o.a = 0, N.unregister(o));
  }, N.register(w, o, o), w;
}
__name(Y, "Y");
function Z(t) {
  t < 132 || (d[t] = x, x = t);
}
__name(Z, "Z");
function y(t) {
  let e = r(t);
  return Z(t), e;
}
__name(y, "y");
function J(t, e, n) {
  let _ = i.fetch(s(t), s(e), s(n));
  return y(_);
}
__name(J, "J");
function tt(t, e) {
  t = t >>> 0;
  let n = u(), _ = [];
  for (let o = t; o < t + 4 * e; o += 4) _.push(y(n.getUint32(o, true)));
  return _;
}
__name(tt, "tt");
function et(t, e) {
  let n = e(t.length * 4, 4) >>> 0, _ = u();
  for (let o = 0; o < t.length; o++) _.setUint32(n + 4 * o, s(t[o]), true);
  return l = t.length, n;
}
__name(et, "et");
function B(t) {
  i.setPanicHook(s(t));
}
__name(B, "B");
function nt(t, e, n) {
  i.__wasm_bindgen_func_elem_1457(t, e, s(n));
}
__name(nt, "nt");
function rt(t, e, n, _) {
  i.__wasm_bindgen_func_elem_600(t, e, s(n), s(_));
}
__name(rt, "rt");
var _t = ["bytes"];
var c = 0;
function G() {
  c++, m = null, A = null, typeof numBytesDecoded < "u" && (numBytesDecoded = 0), typeof l < "u" && (l = 0), typeof d < "u" && (d = new Array(128).fill(void 0), d = d.concat([void 0, null, true, false]), typeof x < "u" && (x = d.length)), i = new WebAssembly.Instance($, K).exports, i.__wbindgen_start();
}
__name(G, "G");
var it = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: t, instance: e }) => {
  e === c && i.__wbg_containerstartupoptions_free(t >>> 0, 1);
});
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, it.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    i.__wbg_containerstartupoptions_free(e, 0);
  }
  get entrypoint() {
    try {
      if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
      let o = i.__wbindgen_add_to_stack_pointer(-16);
      i.__wbg_get_containerstartupoptions_entrypoint(o, this.__wbg_ptr);
      var e = u().getInt32(o + 0, true), n = u().getInt32(o + 4, true), _ = tt(e, n).slice();
      return i.__wbindgen_export4(e, n * 4, 4), _;
    } finally {
      i.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set entrypoint(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let n = et(e, i.__wbindgen_export), _ = l;
    i.__wbg_set_containerstartupoptions_entrypoint(this.__wbg_ptr, n, _);
  }
  get enableInternet() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = i.__wbg_get_containerstartupoptions_enableInternet(this.__wbg_ptr);
    return e === 16777215 ? void 0 : e !== 0;
  }
  set enableInternet(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_containerstartupoptions_enableInternet(this.__wbg_ptr, b(e) ? 16777215 : e ? 1 : 0);
  }
  get env() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = i.__wbg_get_containerstartupoptions_env(this.__wbg_ptr);
    return y(e);
  }
  set env(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_containerstartupoptions_env(this.__wbg_ptr, s(e));
  }
};
Symbol.dispose && (I.prototype[Symbol.dispose] = I.prototype.free);
var ot = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: t, instance: e }) => {
  e === c && i.__wbg_intounderlyingbytesource_free(t >>> 0, 1);
});
var E = class {
  static {
    __name(this, "E");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ot.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    i.__wbg_intounderlyingbytesource_free(e, 0);
  }
  get type() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = i.intounderlyingbytesource_type(this.__wbg_ptr);
    return _t[e];
  }
  get autoAllocateChunkSize() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    return i.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.intounderlyingbytesource_start(this.__wbg_ptr, s(e));
  }
  pull(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let n = i.intounderlyingbytesource_pull(this.__wbg_ptr, s(e));
    return y(n);
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw();
    i.intounderlyingbytesource_cancel(e);
  }
};
Symbol.dispose && (E.prototype[Symbol.dispose] = E.prototype.free);
var st = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: t, instance: e }) => {
  e === c && i.__wbg_intounderlyingsink_free(t >>> 0, 1);
});
var F = class {
  static {
    __name(this, "F");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, st.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    i.__wbg_intounderlyingsink_free(e, 0);
  }
  write(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let n = i.intounderlyingsink_write(this.__wbg_ptr, s(e));
    return y(n);
  }
  close() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw(), n = i.intounderlyingsink_close(e);
    return y(n);
  }
  abort(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let n = this.__destroy_into_raw(), _ = i.intounderlyingsink_abort(n, s(e));
    return y(_);
  }
};
Symbol.dispose && (F.prototype[Symbol.dispose] = F.prototype.free);
var ct = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: t, instance: e }) => {
  e === c && i.__wbg_intounderlyingsource_free(t >>> 0, 1);
});
var j = class {
  static {
    __name(this, "j");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ct.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    i.__wbg_intounderlyingsource_free(e, 0);
  }
  pull(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let n = i.intounderlyingsource_pull(this.__wbg_ptr, s(e));
    return y(n);
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw();
    i.intounderlyingsource_cancel(e);
  }
};
Symbol.dispose && (j.prototype[Symbol.dispose] = j.prototype.free);
var ut = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: t, instance: e }) => {
  e === c && i.__wbg_minifyconfig_free(t >>> 0, 1);
});
var R = class {
  static {
    __name(this, "R");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ut.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    i.__wbg_minifyconfig_free(e, 0);
  }
  get js() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    return i.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_minifyconfig_js(this.__wbg_ptr, e);
  }
  get html() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    return i.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_minifyconfig_html(this.__wbg_ptr, e);
  }
  get css() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    return i.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_minifyconfig_css(this.__wbg_ptr, e);
  }
};
Symbol.dispose && (R.prototype[Symbol.dispose] = R.prototype.free);
var ft = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: t, instance: e }) => {
  e === c && i.__wbg_r2range_free(t >>> 0, 1);
});
var S = class {
  static {
    __name(this, "S");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ft.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    i.__wbg_r2range_free(e, 0);
  }
  get offset() {
    try {
      if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
      let _ = i.__wbindgen_add_to_stack_pointer(-16);
      i.__wbg_get_r2range_offset(_, this.__wbg_ptr);
      var e = u().getInt32(_ + 0, true), n = u().getFloat64(_ + 8, true);
      return e === 0 ? void 0 : n;
    } finally {
      i.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set offset(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_r2range_offset(this.__wbg_ptr, !b(e), b(e) ? 0 : e);
  }
  get length() {
    try {
      if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
      let _ = i.__wbindgen_add_to_stack_pointer(-16);
      i.__wbg_get_r2range_length(_, this.__wbg_ptr);
      var e = u().getInt32(_ + 0, true), n = u().getFloat64(_ + 8, true);
      return e === 0 ? void 0 : n;
    } finally {
      i.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set length(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_r2range_length(this.__wbg_ptr, !b(e), b(e) ? 0 : e);
  }
  get suffix() {
    try {
      if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
      let _ = i.__wbindgen_add_to_stack_pointer(-16);
      i.__wbg_get_r2range_suffix(_, this.__wbg_ptr);
      var e = u().getInt32(_ + 0, true), n = u().getFloat64(_ + 8, true);
      return e === 0 ? void 0 : n;
    } finally {
      i.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set suffix(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_r2range_suffix(this.__wbg_ptr, !b(e), b(e) ? 0 : e);
  }
};
Symbol.dispose && (S.prototype[Symbol.dispose] = S.prototype.free);
var K = { __wbindgen_placeholder__: { __wbg_Error_e83987f665cf5504: /* @__PURE__ */ __name(function(t, e) {
  let n = Error(p(t, e));
  return s(n);
}, "__wbg_Error_e83987f665cf5504"), __wbg_String_8f0eb39a4a4c2f66: /* @__PURE__ */ __name(function(t, e) {
  let n = String(r(e)), _ = k(n, i.__wbindgen_export, i.__wbindgen_export2), o = l;
  u().setInt32(t + 4, o, true), u().setInt32(t + 0, _, true);
}, "__wbg_String_8f0eb39a4a4c2f66"), __wbg___wbindgen_bigint_get_as_i64_f3ebc5a755000afd: /* @__PURE__ */ __name(function(t, e) {
  let n = r(e), _ = typeof n == "bigint" ? n : void 0;
  u().setBigInt64(t + 8, b(_) ? BigInt(0) : _, true), u().setInt32(t + 0, !b(_), true);
}, "__wbg___wbindgen_bigint_get_as_i64_f3ebc5a755000afd"), __wbg___wbindgen_boolean_get_6d5a1ee65bab5f68: /* @__PURE__ */ __name(function(t) {
  let e = r(t), n = typeof e == "boolean" ? e : void 0;
  return b(n) ? 16777215 : n ? 1 : 0;
}, "__wbg___wbindgen_boolean_get_6d5a1ee65bab5f68"), __wbg___wbindgen_debug_string_df47ffb5e35e6763: /* @__PURE__ */ __name(function(t, e) {
  let n = L(r(e)), _ = k(n, i.__wbindgen_export, i.__wbindgen_export2), o = l;
  u().setInt32(t + 4, o, true), u().setInt32(t + 0, _, true);
}, "__wbg___wbindgen_debug_string_df47ffb5e35e6763"), __wbg___wbindgen_in_bb933bd9e1b3bc0f: /* @__PURE__ */ __name(function(t, e) {
  return r(t) in r(e);
}, "__wbg___wbindgen_in_bb933bd9e1b3bc0f"), __wbg___wbindgen_is_bigint_cb320707dcd35f0b: /* @__PURE__ */ __name(function(t) {
  return typeof r(t) == "bigint";
}, "__wbg___wbindgen_is_bigint_cb320707dcd35f0b"), __wbg___wbindgen_is_function_ee8a6c5833c90377: /* @__PURE__ */ __name(function(t) {
  return typeof r(t) == "function";
}, "__wbg___wbindgen_is_function_ee8a6c5833c90377"), __wbg___wbindgen_is_object_c818261d21f283a4: /* @__PURE__ */ __name(function(t) {
  let e = r(t);
  return typeof e == "object" && e !== null;
}, "__wbg___wbindgen_is_object_c818261d21f283a4"), __wbg___wbindgen_is_undefined_2d472862bd29a478: /* @__PURE__ */ __name(function(t) {
  return r(t) === void 0;
}, "__wbg___wbindgen_is_undefined_2d472862bd29a478"), __wbg___wbindgen_jsval_eq_6b13ab83478b1c50: /* @__PURE__ */ __name(function(t, e) {
  return r(t) === r(e);
}, "__wbg___wbindgen_jsval_eq_6b13ab83478b1c50"), __wbg___wbindgen_jsval_loose_eq_b664b38a2f582147: /* @__PURE__ */ __name(function(t, e) {
  return r(t) == r(e);
}, "__wbg___wbindgen_jsval_loose_eq_b664b38a2f582147"), __wbg___wbindgen_number_get_a20bf9b85341449d: /* @__PURE__ */ __name(function(t, e) {
  let n = r(e), _ = typeof n == "number" ? n : void 0;
  u().setFloat64(t + 8, b(_) ? 0 : _, true), u().setInt32(t + 0, !b(_), true);
}, "__wbg___wbindgen_number_get_a20bf9b85341449d"), __wbg___wbindgen_string_get_e4f06c90489ad01b: /* @__PURE__ */ __name(function(t, e) {
  let n = r(e), _ = typeof n == "string" ? n : void 0;
  var o = b(_) ? 0 : k(_, i.__wbindgen_export, i.__wbindgen_export2), w = l;
  u().setInt32(t + 4, w, true), u().setInt32(t + 0, o, true);
}, "__wbg___wbindgen_string_get_e4f06c90489ad01b"), __wbg___wbindgen_throw_b855445ff6a94295: /* @__PURE__ */ __name(function(t, e) {
  throw new Error(p(t, e));
}, "__wbg___wbindgen_throw_b855445ff6a94295"), __wbg__wbg_cb_unref_2454a539ea5790d9: /* @__PURE__ */ __name(function(t) {
  r(t)._wbg_cb_unref();
}, "__wbg__wbg_cb_unref_2454a539ea5790d9"), __wbg_all_fbed0ce6afcc26fb: /* @__PURE__ */ __name(function() {
  return g(function(t) {
    let e = r(t).all();
    return s(e);
  }, arguments);
}, "__wbg_all_fbed0ce6afcc26fb"), __wbg_buffer_ccc4520b36d3ccf4: /* @__PURE__ */ __name(function(t) {
  let e = r(t).buffer;
  return s(e);
}, "__wbg_buffer_ccc4520b36d3ccf4"), __wbg_byobRequest_2344e6975f27456e: /* @__PURE__ */ __name(function(t) {
  let e = r(t).byobRequest;
  return b(e) ? 0 : s(e);
}, "__wbg_byobRequest_2344e6975f27456e"), __wbg_byteLength_bcd42e4025299788: /* @__PURE__ */ __name(function(t) {
  return r(t).byteLength;
}, "__wbg_byteLength_bcd42e4025299788"), __wbg_byteOffset_ca3a6cf7944b364b: /* @__PURE__ */ __name(function(t) {
  return r(t).byteOffset;
}, "__wbg_byteOffset_ca3a6cf7944b364b"), __wbg_call_525440f72fbfc0ea: /* @__PURE__ */ __name(function() {
  return g(function(t, e, n) {
    let _ = r(t).call(r(e), r(n));
    return s(_);
  }, arguments);
}, "__wbg_call_525440f72fbfc0ea"), __wbg_call_e762c39fa8ea36bf: /* @__PURE__ */ __name(function() {
  return g(function(t, e) {
    let n = r(t).call(r(e));
    return s(n);
  }, arguments);
}, "__wbg_call_e762c39fa8ea36bf"), __wbg_cause_2551549fc39b3b73: /* @__PURE__ */ __name(function(t) {
  let e = r(t).cause;
  return s(e);
}, "__wbg_cause_2551549fc39b3b73"), __wbg_cf_909cdf99a01f342e: /* @__PURE__ */ __name(function() {
  return g(function(t) {
    let e = r(t).cf;
    return b(e) ? 0 : s(e);
  }, arguments);
}, "__wbg_cf_909cdf99a01f342e"), __wbg_close_5a6caed3231b68cd: /* @__PURE__ */ __name(function() {
  return g(function(t) {
    r(t).close();
  }, arguments);
}, "__wbg_close_5a6caed3231b68cd"), __wbg_close_6956df845478561a: /* @__PURE__ */ __name(function() {
  return g(function(t) {
    r(t).close();
  }, arguments);
}, "__wbg_close_6956df845478561a"), __wbg_constructor_43c608587565cd11: /* @__PURE__ */ __name(function(t) {
  let e = r(t).constructor;
  return s(e);
}, "__wbg_constructor_43c608587565cd11"), __wbg_done_2042aa2670fb1db1: /* @__PURE__ */ __name(function(t) {
  return r(t).done;
}, "__wbg_done_2042aa2670fb1db1"), __wbg_enqueue_7b18a650aec77898: /* @__PURE__ */ __name(function() {
  return g(function(t, e) {
    r(t).enqueue(r(e));
  }, arguments);
}, "__wbg_enqueue_7b18a650aec77898"), __wbg_entries_e171b586f8f6bdbf: /* @__PURE__ */ __name(function(t) {
  let e = Object.entries(r(t));
  return s(e);
}, "__wbg_entries_e171b586f8f6bdbf"), __wbg_error_6f1d0762f6c8ae2f: /* @__PURE__ */ __name(function(t, e) {
  console.error(r(t), r(e));
}, "__wbg_error_6f1d0762f6c8ae2f"), __wbg_error_a7f8fbb0523dae15: /* @__PURE__ */ __name(function(t) {
  console.error(r(t));
}, "__wbg_error_a7f8fbb0523dae15"), __wbg_exec_527ff84134b6a272: /* @__PURE__ */ __name(function() {
  return g(function(t, e, n) {
    let _ = r(t).exec(p(e, n));
    return s(_);
  }, arguments);
}, "__wbg_exec_527ff84134b6a272"), __wbg_get_7bed016f185add81: /* @__PURE__ */ __name(function(t, e) {
  let n = r(t)[e >>> 0];
  return s(n);
}, "__wbg_get_7bed016f185add81"), __wbg_get_efcb449f58ec27c2: /* @__PURE__ */ __name(function() {
  return g(function(t, e) {
    let n = Reflect.get(r(t), r(e));
    return s(n);
  }, arguments);
}, "__wbg_get_efcb449f58ec27c2"), __wbg_headers_7ae6dbb1272f8fc6: /* @__PURE__ */ __name(function(t) {
  let e = r(t).headers;
  return s(e);
}, "__wbg_headers_7ae6dbb1272f8fc6"), __wbg_instanceof_ArrayBuffer_70beb1189ca63b38: /* @__PURE__ */ __name(function(t) {
  let e;
  try {
    e = r(t) instanceof ArrayBuffer;
  } catch {
    e = false;
  }
  return e;
}, "__wbg_instanceof_ArrayBuffer_70beb1189ca63b38"), __wbg_instanceof_Error_a944ec10920129e2: /* @__PURE__ */ __name(function(t) {
  let e;
  try {
    e = r(t) instanceof Error;
  } catch {
    e = false;
  }
  return e;
}, "__wbg_instanceof_Error_a944ec10920129e2"), __wbg_instanceof_Map_8579b5e2ab5437c7: /* @__PURE__ */ __name(function(t) {
  let e;
  try {
    e = r(t) instanceof Map;
  } catch {
    e = false;
  }
  return e;
}, "__wbg_instanceof_Map_8579b5e2ab5437c7"), __wbg_instanceof_Uint8Array_20c8e73002f7af98: /* @__PURE__ */ __name(function(t) {
  let e;
  try {
    e = r(t) instanceof Uint8Array;
  } catch {
    e = false;
  }
  return e;
}, "__wbg_instanceof_Uint8Array_20c8e73002f7af98"), __wbg_isArray_96e0af9891d0945d: /* @__PURE__ */ __name(function(t) {
  return Array.isArray(r(t));
}, "__wbg_isArray_96e0af9891d0945d"), __wbg_isSafeInteger_d216eda7911dde36: /* @__PURE__ */ __name(function(t) {
  return Number.isSafeInteger(r(t));
}, "__wbg_isSafeInteger_d216eda7911dde36"), __wbg_iterator_e5822695327a3c39: /* @__PURE__ */ __name(function() {
  return s(Symbol.iterator);
}, "__wbg_iterator_e5822695327a3c39"), __wbg_length_69bca3cb64fc8748: /* @__PURE__ */ __name(function(t) {
  return r(t).length;
}, "__wbg_length_69bca3cb64fc8748"), __wbg_length_cdd215e10d9dd507: /* @__PURE__ */ __name(function(t) {
  return r(t).length;
}, "__wbg_length_cdd215e10d9dd507"), __wbg_message_1ee258909d7264fd: /* @__PURE__ */ __name(function(t) {
  let e = r(t).message;
  return s(e);
}, "__wbg_message_1ee258909d7264fd"), __wbg_method_07a9b3454994db22: /* @__PURE__ */ __name(function(t, e) {
  let n = r(e).method, _ = k(n, i.__wbindgen_export, i.__wbindgen_export2), o = l;
  u().setInt32(t + 4, o, true), u().setInt32(t + 0, _, true);
}, "__wbg_method_07a9b3454994db22"), __wbg_name_5383f8ff646a0ac1: /* @__PURE__ */ __name(function(t) {
  let e = r(t).name;
  return s(e);
}, "__wbg_name_5383f8ff646a0ac1"), __wbg_new_1acc0b6eea89d040: /* @__PURE__ */ __name(function() {
  let t = new Object();
  return s(t);
}, "__wbg_new_1acc0b6eea89d040"), __wbg_new_3c3d849046688a66: /* @__PURE__ */ __name(function(t, e) {
  try {
    var n = { a: t, b: e }, _ = /* @__PURE__ */ __name((w, f) => {
      let a = n.a;
      n.a = 0;
      try {
        return rt(a, n.b, w, f);
      } finally {
        n.a = a;
      }
    }, "_");
    let o = new Promise(_);
    return s(o);
  } finally {
    n.a = n.b = 0;
  }
}, "__wbg_new_3c3d849046688a66"), __wbg_new_5a79be3ab53b8aa5: /* @__PURE__ */ __name(function(t) {
  let e = new Uint8Array(r(t));
  return s(e);
}, "__wbg_new_5a79be3ab53b8aa5"), __wbg_new_9edf9838a2def39c: /* @__PURE__ */ __name(function() {
  return g(function() {
    let t = new Headers();
    return s(t);
  }, arguments);
}, "__wbg_new_9edf9838a2def39c"), __wbg_new_a7442b4b19c1a356: /* @__PURE__ */ __name(function(t, e) {
  let n = new Error(p(t, e));
  return s(n);
}, "__wbg_new_a7442b4b19c1a356"), __wbg_new_no_args_ee98eee5275000a4: /* @__PURE__ */ __name(function(t, e) {
  let n = new Function(p(t, e));
  return s(n);
}, "__wbg_new_no_args_ee98eee5275000a4"), __wbg_new_with_byte_offset_and_length_46e3e6a5e9f9e89b: /* @__PURE__ */ __name(function(t, e, n) {
  let _ = new Uint8Array(r(t), e >>> 0, n >>> 0);
  return s(_);
}, "__wbg_new_with_byte_offset_and_length_46e3e6a5e9f9e89b"), __wbg_new_with_headers_9e53e9ff677bca2a: /* @__PURE__ */ __name(function() {
  return g(function(t) {
    let e = new Headers(r(t));
    return s(e);
  }, arguments);
}, "__wbg_new_with_headers_9e53e9ff677bca2a"), __wbg_new_with_length_01aa0dc35aa13543: /* @__PURE__ */ __name(function(t) {
  let e = new Uint8Array(t >>> 0);
  return s(e);
}, "__wbg_new_with_length_01aa0dc35aa13543"), __wbg_new_with_opt_buffer_source_and_init_d7e792cdf59c8ea6: /* @__PURE__ */ __name(function() {
  return g(function(t, e) {
    let n = new Response(r(t), r(e));
    return s(n);
  }, arguments);
}, "__wbg_new_with_opt_buffer_source_and_init_d7e792cdf59c8ea6"), __wbg_new_with_opt_readable_stream_and_init_b3dac7204db32cac: /* @__PURE__ */ __name(function() {
  return g(function(t, e) {
    let n = new Response(r(t), r(e));
    return s(n);
  }, arguments);
}, "__wbg_new_with_opt_readable_stream_and_init_b3dac7204db32cac"), __wbg_new_with_opt_str_and_init_271896583401be6f: /* @__PURE__ */ __name(function() {
  return g(function(t, e, n) {
    let _ = new Response(t === 0 ? void 0 : p(t, e), r(n));
    return s(_);
  }, arguments);
}, "__wbg_new_with_opt_str_and_init_271896583401be6f"), __wbg_next_020810e0ae8ebcb0: /* @__PURE__ */ __name(function() {
  return g(function(t) {
    let e = r(t).next();
    return s(e);
  }, arguments);
}, "__wbg_next_020810e0ae8ebcb0"), __wbg_next_2c826fe5dfec6b6a: /* @__PURE__ */ __name(function(t) {
  let e = r(t).next;
  return s(e);
}, "__wbg_next_2c826fe5dfec6b6a"), __wbg_prepare_d43cf26ba77cf188: /* @__PURE__ */ __name(function() {
  return g(function(t, e, n) {
    let _ = r(t).prepare(p(e, n));
    return s(_);
  }, arguments);
}, "__wbg_prepare_d43cf26ba77cf188"), __wbg_prototypesetcall_2a6620b6922694b2: /* @__PURE__ */ __name(function(t, e, n) {
  Uint8Array.prototype.set.call(q(t, e), r(n));
}, "__wbg_prototypesetcall_2a6620b6922694b2"), __wbg_queueMicrotask_34d692c25c47d05b: /* @__PURE__ */ __name(function(t) {
  let e = r(t).queueMicrotask;
  return s(e);
}, "__wbg_queueMicrotask_34d692c25c47d05b"), __wbg_queueMicrotask_9d76cacb20c84d58: /* @__PURE__ */ __name(function(t) {
  queueMicrotask(r(t));
}, "__wbg_queueMicrotask_9d76cacb20c84d58"), __wbg_resolve_caf97c30b83f7053: /* @__PURE__ */ __name(function(t) {
  let e = Promise.resolve(r(t));
  return s(e);
}, "__wbg_resolve_caf97c30b83f7053"), __wbg_respond_0f4dbf5386f5c73e: /* @__PURE__ */ __name(function() {
  return g(function(t, e) {
    r(t).respond(e >>> 0);
  }, arguments);
}, "__wbg_respond_0f4dbf5386f5c73e"), __wbg_results_66739944b6791c24: /* @__PURE__ */ __name(function() {
  return g(function(t) {
    let e = r(t).results;
    return b(e) ? 0 : s(e);
  }, arguments);
}, "__wbg_results_66739944b6791c24"), __wbg_set_8b342d8cd9d2a02c: /* @__PURE__ */ __name(function() {
  return g(function(t, e, n, _, o) {
    r(t).set(p(e, n), p(_, o));
  }, arguments);
}, "__wbg_set_8b342d8cd9d2a02c"), __wbg_set_9e6516df7b7d0f19: /* @__PURE__ */ __name(function(t, e, n) {
  r(t).set(q(e, n));
}, "__wbg_set_9e6516df7b7d0f19"), __wbg_set_c2abbebe8b9ebee1: /* @__PURE__ */ __name(function() {
  return g(function(t, e, n) {
    return Reflect.set(r(t), r(e), r(n));
  }, arguments);
}, "__wbg_set_c2abbebe8b9ebee1"), __wbg_set_headers_107379072e02fee5: /* @__PURE__ */ __name(function(t, e) {
  r(t).headers = r(e);
}, "__wbg_set_headers_107379072e02fee5"), __wbg_set_status_886bf143c25d0706: /* @__PURE__ */ __name(function(t, e) {
  r(t).status = e;
}, "__wbg_set_status_886bf143c25d0706"), __wbg_static_accessor_GLOBAL_89e1d9ac6a1b250e: /* @__PURE__ */ __name(function() {
  let t = typeof global > "u" ? null : global;
  return b(t) ? 0 : s(t);
}, "__wbg_static_accessor_GLOBAL_89e1d9ac6a1b250e"), __wbg_static_accessor_GLOBAL_THIS_8b530f326a9e48ac: /* @__PURE__ */ __name(function() {
  let t = typeof globalThis > "u" ? null : globalThis;
  return b(t) ? 0 : s(t);
}, "__wbg_static_accessor_GLOBAL_THIS_8b530f326a9e48ac"), __wbg_static_accessor_SELF_6fdf4b64710cc91b: /* @__PURE__ */ __name(function() {
  let t = typeof self > "u" ? null : self;
  return b(t) ? 0 : s(t);
}, "__wbg_static_accessor_SELF_6fdf4b64710cc91b"), __wbg_static_accessor_WINDOW_b45bfc5a37f6cfa2: /* @__PURE__ */ __name(function() {
  let t = typeof window > "u" ? null : window;
  return b(t) ? 0 : s(t);
}, "__wbg_static_accessor_WINDOW_b45bfc5a37f6cfa2"), __wbg_then_4f46f6544e6b4a28: /* @__PURE__ */ __name(function(t, e) {
  let n = r(t).then(r(e));
  return s(n);
}, "__wbg_then_4f46f6544e6b4a28"), __wbg_then_70d05cf780a18d77: /* @__PURE__ */ __name(function(t, e, n) {
  let _ = r(t).then(r(e), r(n));
  return s(_);
}, "__wbg_then_70d05cf780a18d77"), __wbg_toString_8eec07f6f4c057e4: /* @__PURE__ */ __name(function(t) {
  let e = r(t).toString();
  return s(e);
}, "__wbg_toString_8eec07f6f4c057e4"), __wbg_url_3e15bfb59fa6b660: /* @__PURE__ */ __name(function(t, e) {
  let n = r(e).url, _ = k(n, i.__wbindgen_export, i.__wbindgen_export2), o = l;
  u().setInt32(t + 4, o, true), u().setInt32(t + 0, _, true);
}, "__wbg_url_3e15bfb59fa6b660"), __wbg_value_692627309814bb8c: /* @__PURE__ */ __name(function(t) {
  let e = r(t).value;
  return s(e);
}, "__wbg_value_692627309814bb8c"), __wbg_view_f6c15ac9fed63bbd: /* @__PURE__ */ __name(function(t) {
  let e = r(t).view;
  return b(e) ? 0 : s(e);
}, "__wbg_view_f6c15ac9fed63bbd"), __wbindgen_cast_2241b6af4c4b2941: /* @__PURE__ */ __name(function(t, e) {
  let n = p(t, e);
  return s(n);
}, "__wbindgen_cast_2241b6af4c4b2941"), __wbindgen_cast_4625c577ab2ec9ee: /* @__PURE__ */ __name(function(t) {
  let e = BigInt.asUintN(64, t);
  return s(e);
}, "__wbindgen_cast_4625c577ab2ec9ee"), __wbindgen_cast_9ae0607507abb057: /* @__PURE__ */ __name(function(t) {
  return s(t);
}, "__wbindgen_cast_9ae0607507abb057"), __wbindgen_cast_9d7b003571fd2c19: /* @__PURE__ */ __name(function(t, e) {
  let n = Y(t, e, i.__wasm_bindgen_func_elem_1456, nt);
  return s(n);
}, "__wbindgen_cast_9d7b003571fd2c19"), __wbindgen_object_clone_ref: /* @__PURE__ */ __name(function(t) {
  let e = r(t);
  return s(e);
}, "__wbindgen_object_clone_ref"), __wbindgen_object_drop_ref: /* @__PURE__ */ __name(function(t) {
  y(t);
}, "__wbindgen_object_drop_ref") } };
var at = new WebAssembly.Instance($, K);
i = at.exports;
Error.stackTraceLimit = 100;
var z = false;
function Q() {
  B && B(function(t) {
    let e = new Error("Rust panic: " + t);
    console.error("Critical", e), z = true;
  });
}
__name(Q, "Q");
Q();
var U = 0;
function D() {
  z && (console.log("Reinitializing Wasm application"), G(), z = false, Q(), U++);
}
__name(D, "D");
addEventListener("error", (t) => {
  H(t.error);
});
function H(t) {
  t instanceof WebAssembly.RuntimeError && (console.error("Critical", t), z = true);
}
__name(H, "H");
var M = class extends gt {
  static {
    __name(this, "M");
  }
};
M.prototype.fetch = function(e) {
  return J.call(this, e, this.env, this.ctx);
};
var wt = { set: /* @__PURE__ */ __name((t, e, n, _) => Reflect.set(t.instance, e, n, _), "set"), has: /* @__PURE__ */ __name((t, e) => Reflect.has(t.instance, e), "has"), deleteProperty: /* @__PURE__ */ __name((t, e) => Reflect.deleteProperty(t.instance, e), "deleteProperty"), apply: /* @__PURE__ */ __name((t, e, n) => Reflect.apply(t.instance, e, n), "apply"), construct: /* @__PURE__ */ __name((t, e, n) => Reflect.construct(t.instance, e, n), "construct"), getPrototypeOf: /* @__PURE__ */ __name((t) => Reflect.getPrototypeOf(t.instance), "getPrototypeOf"), setPrototypeOf: /* @__PURE__ */ __name((t, e) => Reflect.setPrototypeOf(t.instance, e), "setPrototypeOf"), isExtensible: /* @__PURE__ */ __name((t) => Reflect.isExtensible(t.instance), "isExtensible"), preventExtensions: /* @__PURE__ */ __name((t) => Reflect.preventExtensions(t.instance), "preventExtensions"), getOwnPropertyDescriptor: /* @__PURE__ */ __name((t, e) => Reflect.getOwnPropertyDescriptor(t.instance, e), "getOwnPropertyDescriptor"), defineProperty: /* @__PURE__ */ __name((t, e, n) => Reflect.defineProperty(t.instance, e, n), "defineProperty"), ownKeys: /* @__PURE__ */ __name((t) => Reflect.ownKeys(t.instance), "ownKeys") };
var v = { construct(t, e, n) {
  try {
    D();
    let _ = { instance: Reflect.construct(t, e, n), instanceId: U, ctor: t, args: e, newTarget: n };
    return new Proxy(_, { ...wt, get(o, w, f) {
      o.instanceId !== U && (o.instance = Reflect.construct(o.ctor, o.args, o.newTarget), o.instanceId = U);
      let a = Reflect.get(o.instance, w, f);
      return typeof a != "function" ? a : a.constructor === Function ? new Proxy(a, { apply(h, C, T) {
        D();
        try {
          return h.apply(C, T);
        } catch (W) {
          throw H(W), W;
        }
      } }) : new Proxy(a, { async apply(h, C, T) {
        D();
        try {
          return await h.apply(C, T);
        } catch (W) {
          throw H(W), W;
        }
      } });
    } });
  } catch (_) {
    throw z = true, _;
  }
} };
var pt = new Proxy(M, v);
var ht = new Proxy(I, v);
var yt = new Proxy(E, v);
var mt = new Proxy(F, v);
var xt = new Proxy(j, v);
var vt = new Proxy(R, v);
var It = new Proxy(S, v);

// ../../../../.nvm/versions/node/v24.11.0/lib/node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts
var drainBody = /* @__PURE__ */ __name(async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } finally {
    try {
      if (request.body !== null && !request.bodyUsed) {
        const reader = request.body.getReader();
        while (!(await reader.read()).done) {
        }
      }
    } catch (e) {
      console.error("Failed to drain the unused request body.", e);
    }
  }
}, "drainBody");
var middleware_ensure_req_body_drained_default = drainBody;

// ../../../../.nvm/versions/node/v24.11.0/lib/node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts
function reduceError(e) {
  return {
    name: e?.name,
    message: e?.message ?? String(e),
    stack: e?.stack,
    cause: e?.cause === void 0 ? void 0 : reduceError(e.cause)
  };
}
__name(reduceError, "reduceError");
var jsonError = /* @__PURE__ */ __name(async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } catch (e) {
    const error = reduceError(e);
    return Response.json(error, {
      status: 500,
      headers: { "MF-Experimental-Error-Stack": "true" }
    });
  }
}, "jsonError");
var middleware_miniflare3_json_error_default = jsonError;

// .wrangler/tmp/bundle-5eje84/middleware-insertion-facade.js
var __INTERNAL_WRANGLER_MIDDLEWARE__ = [
  middleware_ensure_req_body_drained_default,
  middleware_miniflare3_json_error_default
];
var middleware_insertion_facade_default = pt;

// ../../../../.nvm/versions/node/v24.11.0/lib/node_modules/wrangler/templates/middleware/common.ts
var __facade_middleware__ = [];
function __facade_register__(...args) {
  __facade_middleware__.push(...args.flat());
}
__name(__facade_register__, "__facade_register__");
function __facade_invokeChain__(request, env, ctx, dispatch, middlewareChain) {
  const [head, ...tail] = middlewareChain;
  const middlewareCtx = {
    dispatch,
    next(newRequest, newEnv) {
      return __facade_invokeChain__(newRequest, newEnv, ctx, dispatch, tail);
    }
  };
  return head(request, env, ctx, middlewareCtx);
}
__name(__facade_invokeChain__, "__facade_invokeChain__");
function __facade_invoke__(request, env, ctx, dispatch, finalMiddleware) {
  return __facade_invokeChain__(request, env, ctx, dispatch, [
    ...__facade_middleware__,
    finalMiddleware
  ]);
}
__name(__facade_invoke__, "__facade_invoke__");

// .wrangler/tmp/bundle-5eje84/middleware-loader.entry.ts
var __Facade_ScheduledController__ = class ___Facade_ScheduledController__ {
  constructor(scheduledTime, cron, noRetry) {
    this.scheduledTime = scheduledTime;
    this.cron = cron;
    this.#noRetry = noRetry;
  }
  static {
    __name(this, "__Facade_ScheduledController__");
  }
  #noRetry;
  noRetry() {
    if (!(this instanceof ___Facade_ScheduledController__)) {
      throw new TypeError("Illegal invocation");
    }
    this.#noRetry();
  }
};
function wrapExportedHandler(worker) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return worker;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  const fetchDispatcher = /* @__PURE__ */ __name(function(request, env, ctx) {
    if (worker.fetch === void 0) {
      throw new Error("Handler does not export a fetch() function.");
    }
    return worker.fetch(request, env, ctx);
  }, "fetchDispatcher");
  return {
    ...worker,
    fetch(request, env, ctx) {
      const dispatcher = /* @__PURE__ */ __name(function(type, init) {
        if (type === "scheduled" && worker.scheduled !== void 0) {
          const controller = new __Facade_ScheduledController__(
            Date.now(),
            init.cron ?? "",
            () => {
            }
          );
          return worker.scheduled(controller, env, ctx);
        }
      }, "dispatcher");
      return __facade_invoke__(request, env, ctx, dispatcher, fetchDispatcher);
    }
  };
}
__name(wrapExportedHandler, "wrapExportedHandler");
function wrapWorkerEntrypoint(klass) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return klass;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  return class extends klass {
    #fetchDispatcher = /* @__PURE__ */ __name((request, env, ctx) => {
      this.env = env;
      this.ctx = ctx;
      if (super.fetch === void 0) {
        throw new Error("Entrypoint class does not define a fetch() function.");
      }
      return super.fetch(request);
    }, "#fetchDispatcher");
    #dispatcher = /* @__PURE__ */ __name((type, init) => {
      if (type === "scheduled" && super.scheduled !== void 0) {
        const controller = new __Facade_ScheduledController__(
          Date.now(),
          init.cron ?? "",
          () => {
          }
        );
        return super.scheduled(controller);
      }
    }, "#dispatcher");
    fetch(request) {
      return __facade_invoke__(
        request,
        this.env,
        this.ctx,
        this.#dispatcher,
        this.#fetchDispatcher
      );
    }
  };
}
__name(wrapWorkerEntrypoint, "wrapWorkerEntrypoint");
var WRAPPED_ENTRY;
if (typeof middleware_insertion_facade_default === "object") {
  WRAPPED_ENTRY = wrapExportedHandler(middleware_insertion_facade_default);
} else if (typeof middleware_insertion_facade_default === "function") {
  WRAPPED_ENTRY = wrapWorkerEntrypoint(middleware_insertion_facade_default);
}
var middleware_loader_entry_default = WRAPPED_ENTRY;
export {
  ht as ContainerStartupOptions,
  yt as IntoUnderlyingByteSource,
  mt as IntoUnderlyingSink,
  xt as IntoUnderlyingSource,
  vt as MinifyConfig,
  It as R2Range,
  __INTERNAL_WRANGLER_MIDDLEWARE__,
  middleware_loader_entry_default as default
};
//# sourceMappingURL=index.js.map

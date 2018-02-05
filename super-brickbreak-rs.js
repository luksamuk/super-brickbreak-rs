"use strict";

if( typeof Rust === 'undefined' ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        factory();
    }
}( this, function() {
    const Module = {};
    let HEAP8 = null;
    let HEAP16 = null;
    let HEAP32 = null;
    let HEAPU8 = null;
    let HEAPU16 = null;
    let HEAPU32 = null;
    let HEAPF32 = null;
    let HEAPF64 = null;

    Object.defineProperty( Module, 'nodejs', { value: (typeof window === 'undefined') } );
    Object.defineProperty( Module, 'exports', { value: {} } );

    const __imports = {
        env: {
            "__extjs_de942ef9ccd064c41dc92d5b5bf83c61aeb00278": function($0) {
                Module.STDWEB.increment_refcount( $0 );
            },
            "__extjs_d8a439451216bbc6cd9f3012f189d2ad6a2e9459": function($0) {
                Module.STDWEB.decrement_refcount( $0 );
            },
            "__extjs_4e6af0d6e88fcc010da73a6a95a15890aa822146": function($0) {
                Module.STDWEB.from_js($0, (function(){return window.innerWidth})());
            },
            "__extjs_08e2b636284d8f82c2c9b8ab5f60e7b575c3ef5b": function($0) {
                Module.STDWEB.from_js($0, (function(){return window.innerHeight})());
            },
            "__extjs_a5a563883f23ccc4cbb70c52ca95cd7e9eabb17c": function($0, $1) {
                $1 = Module.STDWEB.to_js($1);Module.STDWEB.from_js($0, (function(){return ($1). getContext ("2d");})());
            },
            "__extjs_55e6c3e9bbb448e8fa91af21edcd92623cc8a66c": function($0) {
                Module.STDWEB.from_js($0, (function(){var img = new Image (); img.src = "./sphere.png" ; return img ;})());
            },
            "__extjs_988e45f9234929e51115534dd4e92d93cb64b62c": function($0) {
                Module.STDWEB.from_js($0, (function(){var img = new Image (); img.src = "./paddle.png" ; return img ;})());
            },
            "__extjs_b284cce5501a85daf64a747ff750fc730ee38c0a": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){($1). font = (14 * ($2). width / 720)+ "px GohuFont" ;})());
            },
            "__extjs_25e6f91f4041a3f7e577444323c3218ed8db3aaa": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){($1). clearRect (0 , 0 , ($2), ($3));})());
            },
            "__extjs_1a95850e5a47e94d8c6c14be45be31748ceaf8d0": function($0, $1, $2, $3, $4, $5, $6) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);$6 = Module.STDWEB.to_js($6);Module.STDWEB.from_js($0, (function(){($1). drawImage (($2), ($3), ($4), ($5), ($6));})());
            },
            "__extjs_487dbf4f5d76818a09a30013b7c4ac4618041ee7": function($0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);$6 = Module.STDWEB.to_js($6);$7 = Module.STDWEB.to_js($7);$8 = Module.STDWEB.to_js($8);$9 = Module.STDWEB.to_js($9);Module.STDWEB.from_js($0, (function(){($1). beginPath (); ($2). arc (($3), ($4), ($5), 0 , Math.PI * 2.0); ($6). fillStyle = ($7); ($8). fill (); ($9). closePath ();})());
            },
            "__extjs_3cb19c4ae669de9b893c5c4ab336b8aa7ca1a522": function($0, $1, $2, $3, $4, $5, $6, $7, $8) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);$6 = Module.STDWEB.to_js($6);$7 = Module.STDWEB.to_js($7);$8 = Module.STDWEB.to_js($8);Module.STDWEB.from_js($0, (function(){($1). fillStyle = ($2); ($3). textAlign = ($4); ($5). fillText (($6), ($7), ($8));})());
            },
            "__extjs_7b22db4c6833c809be11754e05b2228ccdee97ef": function($0, $1, $2, $3, $4, $5, $6) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);$6 = Module.STDWEB.to_js($6);Module.STDWEB.from_js($0, (function(){var ctx = ($1); var pos_x = ($2); var pos_y = ($3); var size_x = ($4)/ 2.0 ; var size_y = ($5)/ 2.0 ; ctx.beginPath (); ctx.moveTo (pos_x - size_x , pos_y - size_y); ctx.lineTo (pos_x , pos_y); ctx.lineTo (pos_x + size_x , pos_y - size_y); ctx.fillStyle = ($6); ctx.fill (); ctx.closePath (); ctx.beginPath (); ctx.moveTo (pos_x - size_x , pos_y + size_y); ctx.lineTo (pos_x , pos_y); ctx.lineTo (pos_x + size_x , pos_y + size_y); ctx.fillStyle = "#666" ; ctx.fill (); ctx.closePath (); ctx.beginPath (); ctx.moveTo (pos_x - size_x , pos_y - size_y); ctx.lineTo (pos_x , pos_y); ctx.lineTo (pos_x - size_x , pos_y + size_y); ctx.fillStyle = "#AAA" ; ctx.fill (); ctx.closePath (); ctx.beginPath (); ctx.moveTo (pos_x + size_x , pos_y - size_y); ctx.lineTo (pos_x , pos_y); ctx.lineTo (pos_x + size_x , pos_y + size_y); ctx.fillStyle = "#AAA" ; ctx.fill (); ctx.closePath ();})());
            },
            "__extjs_4074f3126a748dd0531b234a849f421316b7f8fa": function($0, $1, $2, $3, $4) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);Module.STDWEB.from_js($0, (function(){($1). width = ($2); ($3). height = ($4);})());
            },
            "__extjs_a854b46b0130157ed04e83628fc1010d5c0dcffd": function($0, $1, $2, $3, $4) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);Module.STDWEB.from_js($0, (function(){if (typeof document.webkitCancelFullScreen !=="undefined"){if (($1)){console.log ("Exiting fullscreen mode"); document.webkitCancelFullScreen ();}else {console.log ("Entering fullscreen mode"); ($2). webkitRequestFullScreen (Element.ALLOW_KEYBOARD_INPUT);}}else if (typeof document.mozCancelFullScreen !=="undefined"){if (($3)){console.log ("Exiting fullscreen mode"); document.mozCancelFullScreen ();}else {console.log ("Entering fullscreen mode"); ($4). mozRequestFullScreen ();}}})());
            },
            "__extjs_f0e20ef326a4c975a175ada3dd97c96e44ad2b14": function($0) {
                Module.STDWEB.from_js($0, (function(){return (window.innerHeight > window.innerWidth);})());
            },
            "__extjs_0bd21f415a09e557f6e24a07d35bd26996f960a5": function($0) {
                Module.STDWEB.from_js($0, (function(){return 67.5 + (Math.random ()* 46);})());
            },
            "__extjs_cf8ef57870d176e111428c0ad472311c5c8d5d07": function($0) {
                Module.STDWEB.from_js($0, (function(){return window ;})());
            },
            "__extjs_d12a8a2eb9b1308d78c85a774baa5ccf56165b6a": function($0) {
                Module.STDWEB.from_js($0, (function(){return document ;})());
            },
            "__extjs_798df3b3e33592ba1fb77845033bc0b4316ce74f": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){return ($1). getElementById (($2));})());
            },
            "__extjs_d0f9580b9cfe82e2ee67d3707e52d87bbabe59f2": function() {
                Module.STDWEB = {};
            },
            "__extjs_4985c7263834081d123cc7eff225fe2010747092": function() {
                Module.STDWEB.alloc = Module.web_malloc ; Module.STDWEB.dyncall = function (signature , ptr , args){return Module.web_table.get (ptr). apply (null , args);}; Module.STDWEB.utf8_len = function utf8_len (str){let len = 0 ; for (let i = 0 ; i < str.length ; ++i){let u = str.charCodeAt (i); if (u >= 0xD800 && u <= 0xDFFF){u = 0x10000 + ((u & 0x3FF)<< 10)| (str.charCodeAt (++i)& 0x3FF);}if (u <= 0x7F){++len ;}else if (u <= 0x7FF){len += 2 ;}else if (u <= 0xFFFF){len += 3 ;}else if (u <= 0x1FFFFF){len += 4 ;}else if (u <= 0x3FFFFFF){len += 5 ;}else {len += 6 ;}}return len ;};
            },
            "__extjs_a986a787f7d7d1abc8c97008ceb5c6945d3f620f": function() {
                Module.STDWEB.to_utf8 = function to_utf8 (str , addr){for (var i = 0 ; i < str.length ; ++i){var u = str.charCodeAt (i); if (u >= 0xD800 && u <= 0xDFFF){u = 0x10000 + ((u & 0x3FF)<< 10)| (str.charCodeAt (++i)& 0x3FF);}if (u <= 0x7F){HEAPU8 [addr ++]= u ;}else if (u <= 0x7FF){HEAPU8 [addr ++]= 0xC0 | (u >> 6); HEAPU8 [addr ++]= 0x80 | (u & 63);}else if (u <= 0xFFFF){HEAPU8 [addr ++]= 0xE0 | (u >> 12); HEAPU8 [addr ++]= 0x80 | ((u >> 6)& 63); HEAPU8 [addr ++]= 0x80 | (u & 63);}else if (u <= 0x1FFFFF){HEAPU8 [addr ++]= 0xF0 | (u >> 18); HEAPU8 [addr ++]= 0x80 | ((u >> 12)& 63); HEAPU8 [addr ++]= 0x80 | ((u >> 6)& 63); HEAPU8 [addr ++]= 0x80 | (u & 63);}else if (u <= 0x3FFFFFF){HEAPU8 [addr ++]= 0xF8 | (u >> 24); HEAPU8 [addr ++]= 0x80 | ((u >> 18)& 63); HEAPU8 [addr ++]= 0x80 | ((u >> 12)& 63); HEAPU8 [addr ++]= 0x80 | ((u >> 6)& 63); HEAPU8 [addr ++]= 0x80 | (u & 63);}else {HEAPU8 [addr ++]= 0xFC | (u >> 30); HEAPU8 [addr ++]= 0x80 | ((u >> 24)& 63); HEAPU8 [addr ++]= 0x80 | ((u >> 18)& 63); HEAPU8 [addr ++]= 0x80 | ((u >> 12)& 63); HEAPU8 [addr ++]= 0x80 | ((u >> 6)& 63); HEAPU8 [addr ++]= 0x80 | (u & 63);}}};
            },
            "__extjs_83c36ea7560e368457b1ae45a44ffef481c2ad44": function() {
                Module.STDWEB.noop = function (){}; Module.STDWEB.to_js = function to_js (address){var kind = HEAPU8 [address + 12]; if (kind ===0){return undefined ;}else if (kind ===1){return null ;}else if (kind ===2){return HEAP32 [address / 4];}else if (kind ===3){return HEAPF64 [address / 8];}else if (kind ===4){var pointer = HEAPU32 [address / 4]; var length = HEAPU32 [(address + 4)/ 4]; return Module.STDWEB.to_js_string (pointer , length);}else if (kind ===5){return false ;}else if (kind ===6){return true ;}else if (kind ===7){var pointer = HEAPU32 [address / 4]; var length = HEAPU32 [(address + 4)/ 4]; var output = []; for (var i = 0 ; i < length ; ++i){output.push (Module.STDWEB.to_js (pointer + i * 16));}return output ;}else if (kind ===8){var value_array_pointer = HEAPU32 [address / 4]; var length = HEAPU32 [(address + 4)/ 4]; var key_array_pointer = HEAPU32 [(address + 8)/ 4]; var output = {}; for (var i = 0 ; i < length ; ++i){var key_pointer = HEAPU32 [(key_array_pointer + i * 8)/ 4]; var key_length = HEAPU32 [(key_array_pointer + 4 + i * 8)/ 4]; var key = Module.STDWEB.to_js_string (key_pointer , key_length); var value = Module.STDWEB.to_js (value_array_pointer + i * 16); output [key]= value ;}return output ;}else if (kind ===9 || kind ===11 || kind ===12){return Module.STDWEB.acquire_js_reference (HEAP32 [address / 4]);}else if (kind ===10){var adapter_pointer = HEAPU32 [address / 4]; var pointer = HEAPU32 [(address + 4)/ 4]; var deallocator_pointer = HEAPU32 [(address + 8)/ 4]; var output = function (){if (pointer ===0){throw new ReferenceError ("Already dropped Rust function called!");}var args = Module.STDWEB.alloc (16); Module.STDWEB.serialize_array (args , arguments); Module.STDWEB.dyncall ("vii" , adapter_pointer , [pointer , args]); var result = Module.STDWEB.tmp ; Module.STDWEB.tmp = null ; return result ;}; output.drop = function (){output.drop = Module.STDWEB.noop ; var function_pointer = pointer ; pointer = 0 ; Module.STDWEB.dyncall ("vi" , deallocator_pointer , [function_pointer]);}; return output ;}else if (kind ===13){var adapter_pointer = HEAPU32 [address / 4]; var pointer = HEAPU32 [(address + 4)/ 4]; var deallocator_pointer = HEAPU32 [(address + 8)/ 4]; var output = function (){if (pointer ===0){throw new ReferenceError ("Already called or dropped FnOnce function called!");}output.drop = Module.STDWEB.noop ; var function_pointer = pointer ; pointer = 0 ; var args = Module.STDWEB.alloc (16); Module.STDWEB.serialize_array (args , arguments); Module.STDWEB.dyncall ("vii" , adapter_pointer , [function_pointer , args]); var result = Module.STDWEB.tmp ; Module.STDWEB.tmp = null ; return result ;}; output.drop = function (){output.drop = Module.STDWEB.noop ; var function_pointer = pointer ; pointer = 0 ; Module.STDWEB.dyncall ("vi" , deallocator_pointer , [function_pointer]);}; return output ;}else if (kind ===14){var pointer = HEAPU32 [address / 4]; var length = HEAPU32 [(address + 4)/ 4]; var array_kind = HEAPU32 [(address + 8)/ 4]; var pointer_end = pointer + length ; switch (array_kind){case 0 : return HEAPU8.subarray (pointer , pointer_end); case 1 : return HEAP8.subarray (pointer , pointer_end); case 2 : return HEAPU16.subarray (pointer , pointer_end); case 3 : return HEAP16.subarray (pointer , pointer_end); case 4 : return HEAPU32.subarray (pointer , pointer_end); case 5 : return HEAP32.subarray (pointer , pointer_end); case 6 : return HEAPF32.subarray (pointer , pointer_end); case 7 : return HEAPF64.subarray (pointer , pointer_end);}}};
            },
            "__extjs_2171fbf7dcd6cce3ad90767662e531aee9577813": function() {
                Module.STDWEB.serialize_object = function serialize_object (address , value){var keys = Object.keys (value); var length = keys.length ; var key_array_pointer = Module.STDWEB.alloc (length * 8); var value_array_pointer = Module.STDWEB.alloc (length * 16); HEAPU8 [address + 12]= 8 ; HEAPU32 [address / 4]= value_array_pointer ; HEAPU32 [(address + 4)/ 4]= length ; HEAPU32 [(address + 8)/ 4]= key_array_pointer ; for (var i = 0 ; i < length ; ++i){var key = keys [i]; var key_length = Module.STDWEB.utf8_len (key); var key_pointer = Module.STDWEB.alloc (key_length); Module.STDWEB.to_utf8 (key , key_pointer); var key_address = key_array_pointer + i * 8 ; HEAPU32 [key_address / 4]= key_pointer ; HEAPU32 [(key_address + 4)/ 4]= key_length ; Module.STDWEB.from_js (value_array_pointer + i * 16 , value [key]);}}; Module.STDWEB.serialize_array = function serialize_array (address , value){var length = value.length ; var pointer = Module.STDWEB.alloc (length * 16); HEAPU8 [address + 12]= 7 ; HEAPU32 [address / 4]= pointer ; HEAPU32 [(address + 4)/ 4]= length ; for (var i = 0 ; i < length ; ++i){Module.STDWEB.from_js (pointer + i * 16 , value [i]);}}; Module.STDWEB.from_js = function from_js (address , value){var kind = Object.prototype.toString.call (value); if (kind ==="[object String]"){var length = Module.STDWEB.utf8_len (value); var pointer = 0 ; if (length > 0){pointer = Module.STDWEB.alloc (length); Module.STDWEB.to_utf8 (value , pointer);}HEAPU8 [address + 12]= 4 ; HEAPU32 [address / 4]= pointer ; HEAPU32 [(address + 4)/ 4]= length ;}else if (kind ==="[object Number]"){if (value ===(value | 0)){HEAPU8 [address + 12]= 2 ; HEAP32 [address / 4]= value ;}else {HEAPU8 [address + 12]= 3 ; HEAPF64 [address / 8]= value ;}}else if (value ===null){HEAPU8 [address + 12]= 1 ;}else if (value ===undefined){HEAPU8 [address + 12]= 0 ;}else if (value ===false){HEAPU8 [address + 12]= 5 ;}else if (value ===true){HEAPU8 [address + 12]= 6 ;}else {var refid = Module.STDWEB.acquire_rust_reference (value); var id = 9 ; if (kind ==="[object Object]"){id = 11 ;}else if (kind ==="[object Array]" || kind ==="[object Arguments]"){id = 12 ;}HEAPU8 [address + 12]= id ; HEAP32 [address / 4]= refid ;}};
            },
            "__extjs_8a13e041b26592fd43280496ac01f5f3e049218e": function() {
                Module.STDWEB.to_js_string = function to_js_string (index , length){index = index | 0 ; length = length | 0 ; var end = (index | 0)+ (length | 0); var output = "" ; while (index < end){var x = HEAPU8 [index ++]; if (x < 128){output += String.fromCharCode (x); continue ;}var init = (x & (0x7F >> 2)); var y = 0 ; if (index < end){y = HEAPU8 [index ++];}var ch = (init << 6)| (y & 63); if (x >= 0xE0){var z = 0 ; if (index < end){z = HEAPU8 [index ++];}var y_z = ((y & 63)<< 6)| (z & 63); ch = init << 12 | y_z ; if (x >= 0xF0){var w = 0 ; if (index < end){w = HEAPU8 [index ++];}ch = (init & 7)<< 18 | ((y_z << 6)| (w & 63));}}output += String.fromCharCode (ch); continue ;}return output ;};
            },
            "__extjs_b67f2836bfcab57acb8e21dbe580790ff03192f9": function() {
                var id_to_ref_map = {}; var id_to_refcount_map = {}; var ref_to_id_map = new WeakMap (); var ref_to_id_symbol_map = {}; var last_refid = 1 ; Module.STDWEB.acquire_rust_reference = function (reference){if (reference ===undefined || reference ===null){return 0 ;}var refid = ref_to_id_map.get (reference); if (refid ===undefined){refid = ref_to_id_symbol_map [reference];}if (refid ===undefined){refid = last_refid ++; if (typeof reference ==="symbol"){ref_to_id_symbol_map [reference]= refid ;}else {ref_to_id_map.set (reference , refid);}id_to_ref_map [refid]= reference ; id_to_refcount_map [refid]= 1 ;}else {id_to_refcount_map [refid]++;}return refid ;}; Module.STDWEB.acquire_js_reference = function (refid){return id_to_ref_map [refid];}; Module.STDWEB.increment_refcount = function (refid){id_to_refcount_map [refid]++;}; Module.STDWEB.decrement_refcount = function (refid){id_to_refcount_map [refid]--; if (id_to_refcount_map [refid]===0){var reference = id_to_ref_map [refid]; delete id_to_ref_map [refid]; delete id_to_refcount_map [refid]; if (typeof reference ==="symbol"){delete ref_to_id_symbol_map [reference];}else {ref_to_id_map.delete (reference);}}};
            },
            "__extjs_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf": function() {
                console.error( 'Encountered a panic!' );
            },
            "__extjs_b00b05929b445348eab177b6d3f509bcaa28782e": function($0, $1) {
                console.error( 'Panic error message:', Module.STDWEB.to_js_string( $0, $1 ) );
            },
            "__extjs_20637d8f642203b38c263a5d0f43b9d88ec67c31": function($0, $1, $2) {
                console.error( 'Panic location:', Module.STDWEB.to_js_string( $0, $1 ) + ':' + $2 );
            },
            "__extjs_81d99dd180b9769eb1e2b2849590216cf5c21704": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){var listener = ($1); ($2). addEventListener (($3), listener); return listener ;})());
            },
            "__extjs_a316bcb4c088cc4946fb65fe0972e7956d19f38e": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){var callback = ($1); var request = ($2). requestAnimationFrame (callback); return {request : request , callback : callback , window : ($3)};})());
            },
            "__extjs_3889a204403e4f6787d35f81036b4dcdc5b43ff6": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){console.log ("Key " + ($1)+ ", state: " + ($2));})());
            },
            "__extjs_6e33b5781ef177f98d395307071c5ea678391edb": function($0) {
                Module.STDWEB.from_js($0, (function(){return Date.now ();})());
            },
            "__extjs_0b0957fa3cc35c6c8c25feaf99c45eeacbf1e88b": function($0, $1, $2, $3, $4, $5, $6, $7) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);$6 = Module.STDWEB.to_js($6);$7 = Module.STDWEB.to_js($7);Module.STDWEB.from_js($0, (function(){Module.exports.deviceTiltCallback = ($1); Module.exports.deviceTouchCallback = ($2); if (($3). DeviceMotionEvent){($4). addEventListener ("devicemotion" , function (e){e.preventDefault (); var alpha = 0.0 ; var beta = 0.0 ; var gamma = 0.0 ; alpha += e.rotationRate.alpha ; beta += e.rotationRate.beta ; gamma += e.rotationRate.gamma ; Module.exports.deviceTiltCallback (0.0 , alpha , beta , gamma);}, false);}else {alert ("Sorry, your phone sucks");}if (($5). TouchEvent){($6). addEventListener ("touchstart" , (e)=> {Module.exports.deviceTouchCallback (true);}, false); ($7). addEventListener ("touchend" , (e)=> {Module.exports.deviceTouchCallback (false);}, false);}})());
            },
            "__extjs_affd075eaca85f0b8527a09a29f63ea699040bb6": function($0, $1) {
                $1 = Module.STDWEB.to_js($1);Module.STDWEB.from_js($0, (function(){return ($1). location ;})());
            },
            "__extjs_e9d84225304abfe1f0ac0649a5e9eacd15517749": function($0) {
                return (Module.STDWEB.acquire_js_reference( $0 ) instanceof KeyboardEvent) | 0;
            },
            "__extjs_21ca452112a9fbc71e2e43a589896d83c68b5791": function($0, $1) {
                $1 = Module.STDWEB.to_js($1);Module.STDWEB.from_js($0, (function(){return ($1). key ;})());
            },
            "__extjs_84c33117bbcdea15c2c7cef1b33f9ea9af612b7d": function($0) {
                $0 = Module.STDWEB.to_js($0);($0). preventDefault ();
            },
            "__extjs_ff2c75b4783fd5c9d8c934bbd4a03e66527e05e4": function($0) {
                Module.STDWEB.tmp = Module.STDWEB.to_js( $0 );
            },
            "Math_atan2": function($0, $1) {
                return Math.atan( $0, $1 );
            },
            "cos": function($0) {
                return Math.cos( $0 );
            },
            "cosf": function($0) {
                return Math.cos( $0 );
            },
            "sin": function($0) {
                return Math.sin( $0 );
            },
            "sinf": function($0) {
                return Math.sin( $0 );
            },
            "__web_on_grow": function() {
                const buffer = Module.instance.exports.memory.buffer;
                HEAP8 = new Int8Array( buffer );
                HEAP16 = new Int16Array( buffer );
                HEAP32 = new Int32Array( buffer );
                HEAPU8 = new Uint8Array( buffer );
                HEAPU16 = new Uint16Array( buffer );
                HEAPU32 = new Uint32Array( buffer );
                HEAPF32 = new Float32Array( buffer );
                HEAPF64 = new Float64Array( buffer );
            }
        }
    };

    function __load( instance ) {
        Object.defineProperty( Module, 'instance', { value: instance } );
        Object.defineProperty( Module, 'web_malloc', { value: Module.instance.exports.__web_malloc } );
        Object.defineProperty( Module, 'web_free', { value: Module.instance.exports.__web_free } );
        Object.defineProperty( Module, 'web_table', { value: Module.instance.exports.__web_table } );

        if( typeof module !== 'undefined' && module.exports ) {
            module.exports = Module.exports;
        } else {
            Rust.super_brickbreak_rs.exports = Module.exports;
        }

        __imports.env.__web_on_grow();
        Module.instance.exports.__web_main();
    }

    if( Module.nodejs ) {
        const fs = require( 'fs' );
        const path = require( 'path' );
        const wasm_path = path.join( __dirname, "super-brickbreak-rs.wasm" );
        const buffer = fs.readFileSync( wasm_path );
        const mod = new WebAssembly.Module( buffer );
        const instance = new WebAssembly.Instance( mod, __imports );
        __load( instance );
        return Module.exports;
    } else {
        const __promise = fetch( "super-brickbreak-rs.wasm" )
            .then( response => response.arrayBuffer() )
            .then( bytes => WebAssembly.instantiate( bytes, __imports ) )
            .then( results => {
                __load( results.instance );
                console.log( "Finished loading Rust wasm module 'super_brickbreak_rs'" );
                return Module.exports;
            })
            .catch( error => {
                console.log( "Error loading Rust wasm module 'super_brickbreak_rs':", error );
                throw error;
            });

        Rust.super_brickbreak_rs = __promise;
        return __promise;
    }
}));

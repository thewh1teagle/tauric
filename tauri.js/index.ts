import {
  load,
  DataType,
  open,
  define,
  funcConstructor,
  createPointer,
  freePointer,
  unwrapPointer,
  PointerType,
} from "ffi-rs";

const dynamicLib =
  process.platform === "win32" ? "./tauric.dll" : "libtauric.dylib";
open({ library: "tauric", path: dynamicLib });

const tauric = define({
  run: {
    library: "tauric",
    retType: DataType.I32,
    paramsType: [],
  },
});

const onReadyFunc = () => {
  // free function memory which malloc in c side when it not in use
  console.log('nodejs got ready')
  freePointer({
    paramsType: [
      funcConstructor({
        paramsType: [],
        retType: DataType.Void,
      }),
    ],
    paramsValue: funcExternal,
    pointerType: PointerType.CPointer,
  });
};
const funcExternal = createPointer({
  paramsType: [
    funcConstructor({
      paramsType: [],
      retType: DataType.Void,
    }),
  ],
  paramsValue: [onReadyFunc],
});

async function main() {
  await load({
    library: "tauric",
    funcName: "on_ready",
    runInNewThread: true,
    retType: DataType.Void,
    paramsType: [DataType.External],
    paramsValue: unwrapPointer(funcExternal),
  });
  tauric.run([]);
}

main()
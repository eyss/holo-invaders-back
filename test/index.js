/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example
const path = require("path");
const {
  Orchestrator,
  Config,
  combine,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/holo-invaders-back.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly
  )
});

const dna = Config.dna(dnaPath, "scores");
const config = Config.gen(
  {
    scores: dna
  },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    }
  }
);


orchestrator.registerScenario("create fligth segment", async (s, t) => {
  const { alice } = await s.players({ alice: config }, true);
  const result = await alice.call(
    "scores",
    "scores",
    "profile",
    { name: "hector0513" }
  );
  t.ok(result.Ok);

  await s.consistency()
  const result2 = await alice.call(
    "scores",
    "scores",
    "get_my_profile",
    {}
  );
  t.ok(result2.Ok);
  t.isEqual("hector0513", result2.Ok.name)

});

orchestrator.run();

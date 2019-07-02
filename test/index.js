const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
    // Will print "unhandledRejection err is not defined"
    console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/prediction-game.dna.json")
const dna = Diorama.dna(dnaPath, 'prediction-game')

const diorama = new Diorama({
    instances: {
        alice: dna,
        bob: dna,
    },
    bridges: [],
    debugLog: false,
    executor: tapeExecutor(require('tape')),
    middleware: backwardCompatibilityMiddleware,
})


// <<DEVCAMP>> Your tests here
diorama.registerScenario("Can create a new game", async(s, t, { alice, bob }) => {
    const create_game_result = await alice.callSync("main", "create_game", {
        opponent: bob.agentId,
        timestamp: 0
    })

    console.log(create_game_result);
    t.equal(create_game_result.Ok.length, 46)

});


// <<DEVCAMP>> Your tests here
diorama.registerScenario("Can make a move", async(s, t, { alice, bob }) => {
    const create_game_result = await alice.callSync("main", "create_game", {
        opponent: bob.agentId,
        timestamp: 0
    })

    console.log(create_game_result);
    t.equal(create_game_result.Ok.length, 46)
    console.log("Game on.");

    console.log("Bob kicks off with a suggestion of 12.");
    const move_1_result = await bob.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Suggest: { suggestion: 12 } },
            timestamp: 1,
        }
    })

    t.equal(move_1_result.Err, undefined)

    console.log("Alice is wise to Bob's shenanigans and predicts 12.");
    const move_2_result = await alice.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Guess: { prediction: 12 } },
            timestamp: 2,
        }
    })

    t.equal(move_2_result.Err, undefined);

    console.log("Bob follows up with a suggestion of 7.");
    const move_3_result = await bob.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Suggest: { suggestion: 7 } },
            timestamp: 3,
        }
    })

    t.equal(move_3_result.Err, undefined)

    console.log("Alice is confounded by Bob's shenanigans and predicts 12 again.");
    const move_4_result = await alice.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Guess: { prediction: 12 } },
            timestamp: 4,
        }
    })

    t.equal(move_4_result.Err, undefined);

    console.log("Time to swap!");
    const move_5_result = await bob.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Swap: {} },
            timestamp: 5,
        }
    })

    t.equal(move_5_result.Err, undefined);

    console.log("Bob mistakenly tries to keep suggesting when it isn't his turn.");
    const move_6_result = await bob.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Suggest: { suggestion: 7 } },
            timestamp: 6,
        }
    })

    t.equal(move_6_result.Ok, undefined);

    console.log("Alice makes a suggestion.");
    const move_7_result = await alice.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Suggest: { suggestion: 6 } },
            timestamp: 7,
        }
    });

    t.equal(move_7_result.Err, undefined);

    console.log("Bob makes his first successful prediction.");
    const move_8_result = await alice.callSync('main', 'make_move', {
        new_move: {
            game: create_game_result.Ok,
            move_type: { Guess: { prediction: 6 } },
            timestamp: 8,
        }
    });
    t.equal(move_8_result.Err, undefined);
});

diorama.run()
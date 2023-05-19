use std::path::PathBuf;

use clap::Parser;
use libafl::{
    feedback_or,
    prelude::{
        current_nanos, havoc_mutations, tuple_list, AsSlice, BytesInput, CrashFeedback, ExitKind,
        HasTargetBytes, HitcountsIterableMapObserver, InMemoryCorpus, InProcessExecutor,
        MaxMapFeedback, OnDiskCorpus, SimpleEventManager, SimpleMonitor, StdRand,
        StdScheduledMutator,
    },
    schedulers::QueueScheduler,
    stages::StdMutationalStage,
    state::StdState,
    Fuzzer, StdFuzzer,
};
use libafl_targets::CountersMultiMapObserver;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

extern "Rust" {
    fn decode(encoded_input: &[u8]) -> Vec<u8>;
    fn counters_maps_observer(name: &'static str) -> CountersMultiMapObserver<false>;
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    /// Corpus directory
    corpus: PathBuf,
    #[arg(short, long)]
    /// Corpus directory
    solutions: PathBuf,
}

fn main() {
    let args = Args::parse();
    let counters_observer =
        HitcountsIterableMapObserver::new(unsafe { counters_maps_observer("counters-maps") });
    let counters_feedback = MaxMapFeedback::new(&counters_observer);
    let mut feedback = feedback_or!(counters_feedback);

    let mut objective = CrashFeedback::new();

    // create a State from scratch
    let mut state = StdState::new(
        // RNG
        StdRand::with_seed(current_nanos()),
        // Corpus that will be evolved, we keep it in memory for performance
        InMemoryCorpus::new(),
        // Corpus in which we store solutions (crashes in this example),
        // on disk so the user can get them after stopping the fuzzer
        OnDiskCorpus::new(args.solutions).unwrap(),
        // States of the feedbacks.
        // The feedbacks can report the data that should persist in the State.
        &mut feedback,
        // Same for objective feedbacks
        &mut objective,
    )
    .unwrap();

    // The Monitor trait define how the fuzzer stats are displayed to the user
    let mon = SimpleMonitor::new(|s| println!("{}", s));
    // let mon = TuiMonitor::new(String::from("Baby Fuzzer"), false);

    // The event manager handle the various events generated during the fuzzing loop
    // such as the notification of the addition of a new item to the corpus
    let mut mgr = SimpleEventManager::new(mon);

    // A queue policy to get testcasess from the corpus
    let scheduler = QueueScheduler::new();

    // A fuzzer with feedbacks and a corpus scheduler
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Create the executor for an in-process function with just one observer
    let mut harness = |input: &BytesInput| {
        let target = input.target_bytes();
        let buf = target.as_slice();
        println!("Fuzzing with {:?} ({})", buf, buf.len());
        unsafe { decode(buf) };
        ExitKind::Ok
    };

    let mut executor = InProcessExecutor::new(
        &mut harness,
        tuple_list!(counters_observer),
        &mut fuzzer,
        &mut state,
        &mut mgr,
    )
    .expect("Failed to create the Executor");

    // let mut generator = RandPrintablesGenerator::new(32);

    // Generate 8 initial inputs
    state
        // .generate_initial_inputs(&mut fuzzer, &mut executor, &mut generator, &mut mgr, 8)
        .load_initial_inputs(&mut fuzzer, &mut executor, &mut mgr, &[args.corpus])
        .expect("Failed to generate the initial corpus");

    // Setup a mutational stage with a basic bytes mutator
    let mutator = StdScheduledMutator::new(havoc_mutations());
    let mut stages = tuple_list!(StdMutationalStage::new(mutator));

    fuzzer
        .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
        .expect("Error in the fuzzing loop");
}

#[macro_use]
extern crate criterion;

use std::convert::TryFrom;

use criterion::Criterion;
use criterion::{black_box, BenchmarkId};

use j4rs::{self, Instance, InvocationArg, Jvm};

fn do_instance_creation(jvm: &Jvm) -> Instance {
    jvm.create_instance("org.astonbitecode.j4rs.tests.MyTest", InvocationArg::empty())
        .unwrap()
}

fn do_invocation_w_no_args(jvm: &Jvm, instance: &Instance) -> Instance {
    jvm.invoke(instance, "getMyString", InvocationArg::empty()).unwrap()
}

fn do_invocation_w_string_args(jvm: &Jvm, instance: &Instance) -> Instance {
    jvm.invoke(
        instance,
        "echo",
        &[InvocationArg::try_from("a").unwrap()],
    )
    .unwrap()
}

fn do_invocation_w_integer_args(jvm: &Jvm, instance: &Instance) -> Instance {
    jvm.invoke(
        instance,
        "echo",
        &[InvocationArg::try_from(33_i32).unwrap()],
    )
    .unwrap()
}

fn do_invocation_w_string_args_and_to_rust(jvm: &Jvm, instance: &Instance) {
    let s_instance = jvm
        .invoke(
            instance,
            "getMyWithArgs",
            &[InvocationArg::try_from("a").unwrap()],
        )
        .unwrap();
    let _: String = jvm.to_rust(s_instance).unwrap();
}

fn use_to_rust_deserialized(jvm: &Jvm, instance: &Instance) {
    let i_instance = jvm
        .invoke(
            instance,
            "addInts",
            &[InvocationArg::try_from(30_i32)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
                InvocationArg::try_from(3_i32)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
        )
        .unwrap();
    let _: i32 = jvm.to_rust_deserialized(i_instance).unwrap();
}

fn use_to_rust_boxed(jvm: &Jvm, instance: &Instance) {
    let i_instance = jvm
        .invoke(
            instance,
            "addInts",
            &[InvocationArg::try_from(30_i32)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
                InvocationArg::try_from(3_i32)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
        )
        .unwrap();
    let _: Box<i32> = jvm.to_rust_boxed(i_instance).unwrap();
}

fn j4rs_benchmark(c: &mut Criterion) {
    let jvm: Jvm = j4rs::new_jvm(Vec::new(), Vec::new()).unwrap();
    c.bench_function("instances creation", move |b| {
        b.iter(|| do_instance_creation(black_box(&jvm)))
    });

    let jvm: Jvm = j4rs::new_jvm(Vec::new(), Vec::new()).unwrap();
    let instance = jvm
        .create_instance("org.astonbitecode.j4rs.tests.MyTest", InvocationArg::empty())
        .unwrap();
    c.bench_function("invocations with no args and String result", move |b| {
        b.iter(|| do_invocation_w_no_args(black_box(&jvm), black_box(&instance)))
    });

    let jvm: Jvm = j4rs::new_jvm(Vec::new(), Vec::new()).unwrap();
    let instance = jvm
        .create_instance("org.astonbitecode.j4rs.tests.MyTest", InvocationArg::empty())
        .unwrap();
    c.bench_function("invocations with String arg and String result", move |b| {
        b.iter(|| do_invocation_w_string_args(black_box(&jvm), black_box(&instance)))
    });

    let jvm: Jvm = j4rs::new_jvm(Vec::new(), Vec::new()).unwrap();
    let instance = jvm
        .create_instance("org.astonbitecode.j4rs.tests.MyTest", InvocationArg::empty())
        .unwrap();
    c.bench_function(
        "invocations with Integer arg and Integer result",
        move |b| b.iter(|| do_invocation_w_integer_args(black_box(&jvm), black_box(&instance))),
    );

    let jvm: Jvm = j4rs::new_jvm(Vec::new(), Vec::new()).unwrap();
    let instance = jvm
        .create_instance("org.astonbitecode.j4rs.tests.MyTest", InvocationArg::empty())
        .unwrap();
    c.bench_function(
        "invocations with String arg and String result transformed to Rust",
        move |b| {
            b.iter(|| {
                do_invocation_w_string_args_and_to_rust(black_box(&jvm), black_box(&instance))
            })
        },
    );

    let jvm: Jvm = j4rs::new_jvm(Vec::new(), Vec::new()).unwrap();
    let instance = jvm
        .create_instance("org.astonbitecode.j4rs.tests.MyTest", InvocationArg::empty())
        .unwrap();
    c.bench_function("to_rust_unboxed", move |b| {
        b.iter(|| use_to_rust_deserialized(black_box(&jvm), black_box(&instance)))
    });
}

fn bench_create_java_objects_and_to_rust(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_java_objects_and_to_rust");

    let jvm: Jvm = j4rs::new_jvm(Vec::new(), Vec::new()).unwrap();
    let instance = jvm
        .create_instance("org.astonbitecode.j4rs.tests.MyTest", InvocationArg::empty())
        .unwrap();

    for i in 0..2 {
        group.bench_function(BenchmarkId::new("to_rust_boxed", i), |b| {
            b.iter(|| use_to_rust_boxed(black_box(&jvm), black_box(&instance)))
        });
        group.bench_function(BenchmarkId::new("to_rust_deserialized", i), |b| {
            b.iter(|| use_to_rust_deserialized(black_box(&jvm), black_box(&instance)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    /*j4rs_benchmark,*/ bench_create_java_objects_and_to_rust
);
criterion_main!(benches);

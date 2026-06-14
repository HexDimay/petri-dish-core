use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use petri_dish_core::{cell::body::Body, environment::Environment};

fn create_body() -> Body {
    Body::new()
}

fn benche_create_body(c: &mut Criterion) {
    // let dt = 1.0 / 60.0;
    c.bench_function("Create body", |b| {
        b.iter(|| create_body());
    });
}

fn benche_add_chemical_non_create_body(c: &mut Criterion) {
    let mut body = create_body();
    c.bench_function("Add Glucose without create body", |b| {
        b.iter(|| {
            body.add_chemical(petri_dish_core::cell::chemical::Chemical::Glucose, 100.0);
        });
    });
}

fn benche_add_chemical_create_body(c: &mut Criterion) {
    c.bench_function("Add Glucose with create body", |b| {
        b.iter(|| {
            let mut body = create_body();
            body.add_chemical(petri_dish_core::cell::chemical::Chemical::Glucose, 100.0);
        });
    });
}

fn benche_execute_cmd_glycolysis_body(c: &mut Criterion) {
    let mut body = create_body();
    body.add_chemical(petri_dish_core::cell::chemical::Chemical::Glucose, 100.0);

    c.bench_function("execute_cmd_glycolysis", |b| {
        b.iter(|| {
            body.execute_cmd(
                &petri_dish_core::cell::cmd::CommandBody::Glycolysis,
                &Environment { light: 5.0 },
                1.0 / 60.0,
            );
        });
    });
}

fn benche_execute_cmd_photosynthesis_body(c: &mut Criterion) {
    let mut body = create_body();
    c.bench_function("execute_cmd_photosynthesis", |b| {
        b.iter(|| {
            body.execute_cmd(
                &petri_dish_core::cell::cmd::CommandBody::Photosynthesis,
                &Environment { light: 5.0 },
                1.0 / 60.0,
            );
        });
    });
}

criterion_group!(benches_create_body, benche_create_body,);

criterion_group!(
    benches_add_chemical,
    benche_add_chemical_non_create_body,
    benche_add_chemical_create_body,
);

criterion_group!(
    benches_execute_cmd,
    benche_execute_cmd_glycolysis_body,
    benche_execute_cmd_photosynthesis_body
);

criterion_main!(
    benches_create_body,
    benches_add_chemical,
    benches_execute_cmd
);

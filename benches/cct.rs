use xie::models::CieYuv1960;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xie::observers::{CieObs1931, };
use xie::illuminants::{CctDuv, CctDuvCalc, FL, Ohno2014, Ohno2014Cascade, Robertson};


fn bench_fl1(){
	let _fl1: CieYuv1960<CieObs1931> = CieYuv1960::from(FL::<1>);
}

fn bench_robertson(rob: &Robertson<CieObs1931>){

	let _cct_duv_fl1 = rob.cct_duv(FL::<1>);
}

fn bench_ohno(ohno: &Ohno2014<CieObs1931>){

	let _cct_duv_fl1 = ohno.cct_duv(FL::<1>);
}

fn bench_ohno_cascade(ohno: &Ohno2014Cascade<CieObs1931>){

	let _cct_duv_fl1 = ohno.cct_duv(FL::<1>);
}

fn criterion_benchmark(c: &mut Criterion) {
	let rob: Robertson<CieObs1931> = Robertson::new();
	let ohno: Ohno2014<CieObs1931> = Ohno2014::new();
	let ohno_cascade: Ohno2014Cascade<CieObs1931> = Ohno2014Cascade::new();
	let tds : CctDuv<CieObs1931> = CctDuv::new(vec![[6500.0,-0.001], [6500.0, 0.0495], [6500.0, -0.0495], ]);
	let _yuv: CieYuv1960<_> = tds.clone().into();
    c.bench_function("fl1", |b| b.iter(|| bench_fl1()));
    c.bench_function("robertson", |b| b.iter(|| bench_robertson(black_box(&rob))));
    c.bench_function("ohno 2014", |b| b.iter(|| bench_ohno(black_box(&ohno))));
    c.bench_function("ohno 2014 cascade", |b| b.iter(|| bench_ohno_cascade(black_box(&ohno_cascade))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
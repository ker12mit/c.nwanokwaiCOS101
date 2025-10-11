fn main(){
	// a represents amount
	let a1 = 450_000;
	let a2 = 1_500_000;
	let a3 = 750_000;
	let a4 = 2_850_000;
	let a5 = 250_000;

	// ta represents total amount
	let ta = a1 + a2 + a3 + a4 + a5;

	// q represents quantity
	let q1 = 2;
	let q2 = 1;
	let q3 = 3;
	let q4 = 3;
	let q5 = 1;

	//tq represents total quantity

	let tq = q1 + q2 + q3 + q4 + q5;

	//s represents sum

	let s = tq * ta;

	// avg represents average

	let avg = s / tq; 

	println!("Chief Donatus and Sons Ltd Sales Analysis");
	println!("The sum on the sales record {}",s );
	println!("The average is {}",avg );

}
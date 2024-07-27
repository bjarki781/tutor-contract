//SPDX-License-Identifier: GPL3.0
pragma solidity 0.8.26;


contract TutorContract {
  // is this an optimal size?
  type fxp is int64;

  string[] questions;
  uint8[] answers;
  fxp[] weights;
  fxp[] points;
  uint8 progress;
  fxp grade;

  /*
  enum Error {
      TooSmallAmount,
      TooPoor,
      WrongAnswer,
      InternalError,
  }
  */

  event Answered(bool correct);

  constructor(string[] memory q, uint8[] memory a) {
      questions = q;
      answers = a;
      weights = [fxp.wrap(2000000), fxp.wrap(1500000), fxp.wrap(900000), fxp.wrap(400000), fxp.wrap(200000)];
      points = [fxp.wrap(1000000), fxp.wrap(1000000), fxp.wrap(1000000), fxp.wrap(1000000), fxp.wrap(1000000)];
      progress = 0;
      grade = fxp.wrap(0);
  }

  function fxp_mul(fxp x, fxp y) private pure returns (fxp) {
      return fxp.wrap(fxp.unwrap(x) * fxp.unwrap(y) / 1000000);
  }

  function dot_product(fxp[] memory xs, fxp[] memory ys) private pure returns (fxp) {
      assert(xs.length == ys.length);
      int64 product = 0;
      for (uint8 i = 0; i < xs.length; i++) {
	      product += fxp.unwrap(fxp_mul(xs[i], ys[i]));
      }
      return fxp.wrap(product);
  }

  function calculate_grade() public view returns (fxp) {
      return dot_product(weights, points);
  }

  function get_current_question() public view returns (string memory) {
      return questions[progress];
  }

  /*
  function answer(uint8 a) {
      if (a != answers[progress]) {
	  points.push(-2000000);
	  points.pop();
	  event(Answered false)
      }

      // else if it was correct
      progress++;
      points.push_front(2000000);
      po

  */
}


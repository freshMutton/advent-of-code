import org.scalatest._
import com.advent-of-code-2019.day1._

class Day1Spec extends FunSuite with DiagrammedAssertions {
  test("given a mass of 1969, returns 654") {
    assert(fuelOfMass(1969) == 654)
  }

  test("given a mass of 100756, returns 33583") {
    assert(fuelOfMass(100756) == 33583)
  }
}

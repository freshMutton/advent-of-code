import scala.collection.immutable
import scala.io.Source
import scala.util.parsing.combinator._
import scala.util.matching.Regex

object AdventOfCode extends App {
  //DayOne.solution("/Users/msutton/dev/advent-of-code/2020/day1-input.txt")
  DayTwo.solution("/Users/msutton/dev/advent-of-code/2020/day2-input.txt")
}

object DayOne {
  def findSummingCombination(xs: Vector[Int], n: Int, target: Int) = {
    xs
      .combinations(n)
      .filter(cs => cs.fold(0)(_ + _) == target)
      .flatten
      .toVector
  }

  def calculateExpense(xs: Vector[Int]): Int = {
    xs.fold(1)(_ * _)
  }

  def solution(inputFile: String): Unit = {
    val input = Source
      .fromFile(inputFile)
      .getLines()
      .map(_.toInt)
      .toVector

    // where is my point-free style!?
    println(calculateExpense(findSummingCombination(input, 2, 2020)))

    // part deux - 3 that sum to 2020
    println(calculateExpense(findSummingCombination(input, 3, 2020)))
  }
}

object DayTwo {
  object PolicyPasswordParser extends RegexParsers {
    def separator(sep: Char): Parser[Unit] = sep ^^ { _ => () }
    def number: Parser[Int] = """0|[1-9]\d*""".r ^^ { _.toInt }

    def minMax: Parser[(Int, Int)] = number ~ separator('-') ~ number ^^ {
      case min ~ _ ~ max => (min, max)
    }

    def char: Parser[Char] = """[a-z]""".r ^^ { _.charAt(0) }

    def policy: Parser[PasswordPolicy] = minMax ~ char ^^ {
      case (min, max) ~ char => new PasswordPolicy(min, max, char)
    }

    def string: Parser[String] = """[a-z]+""".r ^^ { _.toString }

    def policyPasswordPair: Parser[(PasswordPolicy, String)] =
      policy ~ separator(':') ~ string ^^ { case policy ~ _ ~ passwd =>
        (policy, passwd)
      }
  }

  class PasswordPolicy(minCount: Int, maxCount: Int, char: Char) {
    def isValidFromOccurances(password: String): Boolean = {
      val occurrances = password.toList.count(c => c == char)

      occurrances >= minCount && occurrances <= maxCount
    }

    def isValidFromPositions(password: String): Boolean = {
      val matchedPositions = Vector(
        password.charAt(minCount - 1),
        password.charAt(maxCount - 1)
      ).count(c => c == char)

      matchedPositions == 1
    }
  }

  def solution(inputFile: String): Unit = {
    val input = Source.fromFile(inputFile).getLines()

    val parsedPasswords = input
      .map(
        PolicyPasswordParser.parse(PolicyPasswordParser.policyPasswordPair, _)
      )
      .toVector

    val result = parsedPasswords
      .count({
        case PolicyPasswordParser.Success(
              (policy: PasswordPolicy, passwd: String),
              _
            ) =>
          policy.isValidFromOccurances(passwd)

        case _ => false
      })

    println(result)

    val result2 = parsedPasswords
      .count({
        case PolicyPasswordParser.Success(
              (policy: PasswordPolicy, passwd: String),
              _
            ) =>
          policy.isValidFromPositions(passwd)

        case _ => false
      })

    println(result2)
  }
}

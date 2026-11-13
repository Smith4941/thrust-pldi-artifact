#!/usr/bin/env python3
"""Tests of tables.py's rules that need no run: `python3 -m unittest test_tables` in this directory."""

import unittest

import tables


class Results(unittest.TestCase):
    """The left-out result columns of D81."""

    def test_an_all_sat_table_leaves_out_its_result_column_and_keeps_a_joined_cell(self):
        body = [([], ["a", "sat", "1.0"]), ([], ["b", "same run as a", None])]
        self.assertEqual(tables.results(body, [(1, 2)]), ["a & 1.0 \\\\", "b & same run as a \\\\"])

    def test_a_joined_cell_after_the_time_shrinks_when_the_result_goes(self):
        body = [([], ["a", "1.0", "sat"]), ([], ["b", "same run as a", None])]
        self.assertEqual(tables.results(body, [(2, 1)]), ["a & 1.0 \\\\", "b & same run as a \\\\"])

    def test_a_table_with_a_row_that_is_not_sat_keeps_every_result_column(self):
        body = [([], ["a", "sat", "1.0", "sat", "2.0"]), ([], ["b", "timeout", "--", "sat", "3.0"])]
        self.assertEqual(tables.results(body, [(1, 2), (3, 4)]),
                         ["a & sat & 1.0 & sat & 2.0 \\\\", "b & timeout & -- & sat & 3.0 \\\\"])

    def test_a_mark_moves_to_the_time(self):
        body = [([], ["a", "sat" + tables.DAGGER, "1.0"])]
        self.assertEqual(tables.results(body, [(1, 2)]), ["a & 1.0" + tables.DAGGER + " \\\\"])

    def test_a_row_without_a_run_keeps_the_result_column(self):
        body = [([], ["a", "sat", "1.0"]), ([], ["b", "body not verified", None])]
        self.assertEqual(tables.results(body, [(1, 2)]),
                         ["a & sat & 1.0 \\\\", "b & \\multicolumn{2}{c}{body not verified} \\\\"])

    def test_a_table_without_a_result_of_its_own_keeps_the_column(self):
        body = [([], ["b", "body not verified", None])]
        self.assertEqual(tables.results(body, [(1, 2)]), ["b & \\multicolumn{2}{c}{body not verified} \\\\"])


class PassSide(unittest.TestCase):
    """A cell is the pass side's result and time (D90); the fail twins are a check outside the cells."""

    @staticmethod
    def rows(*cells):
        rows = [{"id": i, "setting": "", "side": sd, "run": n, "verdict": v, "wall_s": t}
                for i, sd, n, v, t in cells]
        return tables.Rows(rows, {(i, "") for i, sd, _, _, _ in cells if sd == "fail"})

    def test_a_sat_pass_side_gives_its_time_whatever_its_twin_answered(self):
        rows = self.rows(("a", "pass", "1", "sat", "2.0"), ("a", "fail", "1", "timeout", "300.00"),
                         ("a", "pass", "2", "sat", "4.0"), ("a", "fail", "2", "unsat", "1.0"))
        self.assertEqual(tables.verdict_cell(rows, ["a"]), tables.Cell("sat", 3.0, 2, ["1", "2"]))

    def test_a_pass_side_that_is_not_sat_gives_its_own_verdict(self):
        rows = self.rows(("a", "pass", "1", "timeout", "300.00"), ("a", "fail", "1", "unknown", "9.0"))
        self.assertEqual(tables.verdict_cell(rows, ["a"]).result, "timeout")

    def test_a_twin_that_answered_sat_still_stops_it(self):
        rows = self.rows(("a", "pass", "1", "sat", "2.0"), ("a", "fail", "1", "sat", "1.0"))
        with self.assertRaises(SystemExit):
            tables.verdict_cell(rows, ["a"])

    def test_a_missing_twin_row_still_stops_it(self):
        rows = self.rows(("a", "pass", "1", "sat", "2.0"), ("a", "pass", "2", "sat", "2.0"),
                         ("a", "fail", "1", "unsat", "1.0"))
        with self.assertRaises(SystemExit):
            tables.verdict_cell(rows, ["a"])

    def test_the_time_is_the_wall_time_of_the_thrust_command_not_the_solver_calls(self):
        rows = self.rows(("a", "pass", "1", "sat", "2.0"))
        rows[0]["time_s"] = "1.0"
        self.assertEqual(tables.verdict_cell(rows, ["a"]).time, 2.0)

    def test_a_sat_row_without_a_wall_time_stops_it(self):
        rows = self.rows(("a", "pass", "1", "sat", ""))
        with self.assertRaises(SystemExit):
            tables.verdict_cell(rows, ["a"])

    def test_rows_without_the_wall_time_column_stop_it(self):
        with self.assertRaises(SystemExit):
            tables.check_wall_times("rq1.csv", [{"id": "a", "verdict": "sat", "time_s": "1.0"}])
        tables.check_wall_times("rq1.csv", [{"id": "a", "verdict": "sat", "time_s": "1.0", "wall_s": "2.0"}])

    def test_rows_answered_from_a_verdict_cache_stop_it(self):
        row = {"id": "a", "setting": "", "side": "pass", "run": "1", "verdict": "sat", "wall_s": "1.0"}
        tables.check_solved("rq4.csv", [dict(row, verdict_cache="kept", cache_hits="0"), row])
        for cached in (dict(row, verdict_cache="kept", cache_hits="2"), dict(row, verdict_cache="reused", cache_hits="0")):
            with self.assertRaises(SystemExit):
                tables.check_solved("rq4.csv", [cached])

    def test_a_run_without_twin_rows_gives_the_pass_side_s_cell(self):
        rows = self.rows(("a", "pass", "1", "sat", "2.0"), ("a", "pass", "2", "sat", "4.0"))
        rows.twins = {("a", "")}
        self.assertEqual(tables.verdict_cell(rows, ["a"]).time, 3.0)

    def test_a_size_skipped_past_the_cap_prints_no_answer(self):
        rows = self.rows(("a", "pass", "1", "skipped", ""), ("a", "pass", "2", "skipped", ""))
        cell = tables.verdict_cell(rows, ["a"])
        self.assertEqual((cell.time, tables.scalability_text(cell)), (None, "--"))

    def test_a_pass_side_out_of_memory_prints_oom(self):
        rows = self.rows(("a", "pass", "1", "oom", ""))
        self.assertEqual(tables.verdict_cell(rows, ["a"]).result, "OOM")

    def test_an_input_its_list_does_not_mark_printed_stops_it(self):
        rows = tables.Rows([{"id": "a", "setting": "", "side": "pass", "run": "1", "verdict": "sat", "wall_s": "2.0"}],
                           set(), {("a", "", "pass")})
        with self.assertRaises(SystemExit):
            tables.verdict_cell(rows, ["a"])

    def test_the_twins_not_refuted_are_listed_with_their_pass_side(self):
        rows = self.rows(("a", "pass", "1", "sat", "2.0"), ("a", "fail", "1", "timeout", "300.00"),
                         ("b", "pass", "1", "sat", "2.0"), ("b", "fail", "1", "unsat", "1.0"))
        self.assertEqual(tables.unrefuted_twins("rq1.csv", rows), ["rq1.csv: a run 1: fail twin timeout, pass side sat"])

    def test_a_listed_twin_names_every_side_it_guards(self):
        rows = self.rows(("a", "pass", "1", "sat", "2.0"), ("a", "inferred", "1", "timeout", "300.00"),
                         ("a", "fail", "1", "unknown", "9.0"))
        self.assertEqual(tables.unrefuted_twins("rq1.csv", rows),
                         ["rq1.csv: a run 1: fail twin unknown, pass side sat, inferred side timeout"])

    def test_a_sat_twin_stops_it_also_when_no_table_reads_its_input(self):
        rows = self.rows(("unread", "pass", "1", "sat", "2.0"), ("unread", "fail", "1", "sat", "1.0"))
        with self.assertRaises(SystemExit):
            tables.stop_on_sat_twin("rq1.csv", rows)


class Complete(unittest.TestCase):
    """A run holds a row for every printed input and its fail twin, and for every listed one once it ran
    with --diagnostic."""

    FILES = {("a", "", "pass"): "a.rs", ("a", "", "fail"): "a_fail.rs"}
    PRINTED = {("a", "", "pass")}

    @staticmethod
    def rows(*sides):
        return [{"id": "a", "setting": "", "side": sd, "run": "1", "verdict": "sat"} for sd in sides]

    def test_a_run_of_a_printed_row_without_a_listed_twin_is_complete(self):
        tables.check_complete("rq1.csv", self.rows("pass"), {("a", "", "pass"): "a.rs"}, self.PRINTED)

    def test_a_run_lacking_a_printed_row_stops_it(self):
        with self.assertRaises(SystemExit):
            tables.check_complete("rq1.csv", self.rows("fail"), self.FILES, self.PRINTED)

    def test_a_default_run_of_the_printed_rows_and_their_twins_is_complete(self):
        files = {**self.FILES, ("b", "", "pass"): "b.rs", ("b", "", "fail"): "b_fail.rs"}
        tables.check_complete("rq1.csv", self.rows("pass", "fail"), files, self.PRINTED)

    def test_a_default_run_lacking_a_printed_row_s_twin_stops_it(self):
        with self.assertRaises(SystemExit):
            tables.check_complete("rq1.csv", self.rows("pass"), self.FILES, self.PRINTED)

    def test_a_diagnostic_run_lacking_a_listed_row_stops_it(self):
        files = {**self.FILES, ("b", "", "pass"): "b.rs", ("b", "", "fail"): "b_fail.rs"}
        rows = self.rows("pass", "fail") + [{"id": "b", "setting": "", "side": "pass", "run": "1", "verdict": "sat"}]
        with self.assertRaises(SystemExit):
            tables.check_complete("rq1.csv", rows, files, self.PRINTED)


class Inferred(unittest.TestCase):
    """The inferred columns of tab:coroutine: k/n, k counted only when the row's runs verified it."""

    def test_a_verified_row_shows_its_inferred_count_of_its_total(self):
        self.assertEqual(tables.inferred_cell("3", "40", True), "3/40")

    def test_a_row_its_runs_did_not_verify_shows_none_inferred(self):
        self.assertEqual(tables.inferred_cell("1", "1", False), "0/1")

    def test_a_row_without_loops_or_checked_functions_shows_a_dash(self):
        self.assertEqual(tables.inferred_cell("0", "0", True), "--")

    def test_a_row_sharing_an_earlier_row_s_runs_takes_its_verdict(self):
        first = tables.CoroutineRow("a", ["layout", "lemmas"], {})
        second = tables.CoroutineRow("b", ["layout"], {})
        for verified in (True, False):
            cells, shared = tables.coroutine_run([], second, {"a": (first, verified)})
            self.assertEqual((cells, shared), (["same run as a", None], verified))

    def test_result_and_time_stay_the_last_two_columns(self):
        counts = {"functions_verified": "1", "generic": "0", "loc": "10", "spec_lines": "2",
                  "trusted_functions": "0", "ignored_functions": "0", "unchecked_functions": "0", "invariants_inferred": "1",
                  "loops": "1", "contracts_inferred": "1", "functions_checked": "1",
                  "assume_stmts": "0", "generic_contracts_inferred": "0", "generic_functions_checked": "0", "functions_extracted": "0"}
        run = tables.Rows([{"id": "x", "setting": "", "side": "pass", "run": "1", "verdict": "timeout",
                            "wall_s": "300"}], set())
        original = tables.coroutine_rows
        tables.coroutine_rows = lambda: [tables.CoroutineRow("a", ["x"], counts)]
        try:
            lines = tables.coroutine_table(run)
        finally:
            tables.coroutine_rows = original
        self.assertEqual(lines, ["a & 10 & 2 & 0 & 0 & 0 & 0/1 & 0/1 & -- & timeout & -- \\\\"])

    def test_trusted_counts_trusted_and_unchecked_functions_but_not_ignored_ones(self):
        counts = {"functions_verified": "3", "generic": "0", "loc": "10", "spec_lines": "2",
                  "trusted_functions": "4", "ignored_functions": "3", "unchecked_functions": "1",
                  "invariants_inferred": "0", "loops": "0", "contracts_inferred": "0", "functions_checked": "3",
                  "assume_stmts": "0", "generic_contracts_inferred": "0", "generic_functions_checked": "0", "functions_extracted": "0"}
        run = tables.Rows([{"id": "x", "setting": "", "side": "pass", "run": "1", "verdict": "sat",
                            "wall_s": "1.0"}], set())
        original = tables.coroutine_rows
        tables.coroutine_rows = lambda: [tables.CoroutineRow("a", ["x"], counts)]
        try:
            lines = tables.coroutine_table(run)
        finally:
            tables.coroutine_rows = original
        self.assertEqual(lines[0].split(" & ")[4], "2")

    def test_extracted_functions_assumed_facts_and_generic_contracts_have_columns_of_their_own(self):
        counts = {"functions_verified": "4", "generic": "3", "loc": "10", "spec_lines": "2",
                  "trusted_functions": "0", "ignored_functions": "0", "unchecked_functions": "0",
                  "invariants_inferred": "0", "loops": "0", "contracts_inferred": "3", "functions_checked": "4",
                  "assume_stmts": "5", "generic_contracts_inferred": "2", "generic_functions_checked": "3", "functions_extracted": "6"}
        run = tables.Rows([{"id": "x", "setting": "", "side": "pass", "run": "1", "verdict": "sat",
                            "wall_s": "1.0"}], set())
        original = tables.coroutine_rows
        tables.coroutine_rows = lambda: [tables.CoroutineRow("a", ["x"], counts)]
        try:
            lines = tables.coroutine_table(run)
        finally:
            tables.coroutine_rows = original
        self.assertEqual(lines, ["a & 10 & 2 & 6 & 0 & 5 & -- & 1/1 & 2/3 & 1.0 \\\\"])


class CreusotTable(unittest.TestCase):
    """tab:creusot: one LoC column, Creusot's (D109), then each side's spec lines and time; adapters
    left, clients right."""

    CASES = [{"id": "range", "case": "range", "variant": "creusot", "weaker": ""},
             {"id": "all_zero", "case": "all_zero", "variant": "creusot", "weaker": ""}]
    COUNTS = [{"id": "range", "loc": "99", "spec_lines": "16"}, {"id": "all_zero", "loc": "99", "spec_lines": "13"}]
    CREUSOT = [{"case": "range", "name": "Range", "block": "adapter", "loc": "18", "spec": "28", "wall": "0.45"},
               {"case": "all_zero", "name": "all_zero", "block": "client", "loc": "5", "spec": "3", "wall": "9.70"}]

    def test_thrust_loc_is_not_printed_and_an_all_sat_table_drops_the_result_columns(self):
        rows = tables.Rows([{"id": i, "setting": "", "side": "pass", "run": "1", "verdict": "sat", "wall_s": t}
                            for i, t in (("range", "0.5"), ("all_zero", "0.6"))], set())
        original = tables.read
        tables.read = lambda *path: {"cases.csv": self.CASES, "annotations.csv": self.COUNTS,
                                     "creusot.csv": self.CREUSOT}[path[-1]]
        try:
            lines = tables.creusot_table(rows)
        finally:
            tables.read = original
        self.assertEqual([line for line in lines if not line.startswith("%")],
                         ["Range & 18 & 28 & 0.45 & 16 & 0.5 & all\\_zero & 5 & 3 & 9.70 & 13 & 0.6 \\\\"])

    def test_the_shared_trait_prints_its_thrust_count_without_time_and_keeps_the_table_all_sat(self):
        rows = tables.Rows([{"id": "range", "setting": "", "side": "pass", "run": "1", "verdict": "sat", "wall_s": "0.5"}], set())
        original = tables.read
        tables.read = lambda *path: {"cases.csv": self.CASES[:1],
                                     "annotations.csv": self.COUNTS + [{"id": "common", "loc": "4", "spec_lines": "20"}],
                                     "creusot.csv": [{"case": "common", "name": "common", "block": "adapter", "loc": "4",
                                                      "spec": "16", "wall": ""}] + self.CREUSOT[:1]}[path[-1]]
        try:
            lines = tables.creusot_table(rows)
        finally:
            tables.read = original
        self.assertEqual([line for line in lines if not line.startswith("%")],
                         ["common & 4 & 16 & -- & 20 & -- \\\\", "Range & 18 & 28 & 0.45 & 16 & 0.5 \\\\"])


    def test_a_case_without_creusot_s_wall_time_stops_it(self):
        rows = tables.Rows([{"id": "range", "setting": "", "side": "pass", "run": "1", "verdict": "sat", "wall_s": "0.5"}], set())
        original = tables.read
        tables.read = lambda *path: {"cases.csv": self.CASES[:1], "annotations.csv": self.COUNTS,
                                     "creusot.csv": [dict(self.CREUSOT[0], wall="")]}[path[-1]]
        try:
            with self.assertRaises(SystemExit):
                tables.creusot_table(rows)
        finally:
            tables.read = original


class CreusotInferred(unittest.TestCase):
    """tab:creusot-inferred: the labelled cases, those of tab:creusot first, two to a line."""

    CASES = [{"id": "skip", "case": "skip", "block": "adapter", "s5_label": "skip"},
             {"id": "take", "case": "take", "block": "adapter", "s5_label": ""},
             {"id": "collect", "case": "collect", "block": "adapter", "s5_label": "collect"},
             {"id": "fold", "case": "fold", "block": "extra", "s5_label": "fold"}]
    CREUSOT = [{"case": "skip"}, {"case": "take"}]

    def table(self, cells):
        runs = [{"id": c["id"], "side": sd, "file": f"{c['id']}.{sd}.rs"} for c in self.CASES for sd in ("pass", "inferred")]
        rows = tables.Rows([{"id": i, "setting": "", "side": sd, "run": "1", "verdict": v, "wall_s": t}
                            for i, sd, v, t in cells], set())
        original = tables.read
        tables.read = lambda *path: {"cases.csv": self.CASES, "runs.csv": runs, "creusot.csv": self.CREUSOT}[path[-1]]
        try:
            return tables.inferred_table(rows)
        finally:
            tables.read = original

    def test_the_cases_of_tab_creusot_come_first_two_to_a_line_and_an_unlabelled_case_is_left_out(self):
        lines = self.table([("skip", "pass", "sat", "1.0"), ("skip", "inferred", "sat", "2.0"),
                            ("collect", "pass", "sat", "3.0"), ("collect", "inferred", "timeout", "300.00"),
                            ("fold", "pass", "sat", "4.0"), ("fold", "inferred", "sat", "5.0")])
        self.assertEqual(lines, ["% skip: written skip.pass.rs; inferred skip.inferred.rs",
                                 "% collect: written collect.pass.rs; inferred collect.inferred.rs",
                                 "skip & sat & 1.0 & sat & 2.0 & collect & sat & 3.0 & timeout & -- \\\\",
                                 "% fold: written fold.pass.rs; inferred fold.inferred.rs",
                                 "fold & sat & 4.0 & sat & 5.0 \\\\"])

    def test_an_all_sat_table_leaves_out_the_result_columns_of_both_halves(self):
        lines = self.table([("skip", "pass", "sat", "1.0"), ("skip", "inferred", "sat", "2.0"),
                            ("collect", "pass", "sat", "3.0"), ("collect", "inferred", "sat", "6.0"),
                            ("fold", "pass", "sat", "4.0"), ("fold", "inferred", "sat", "5.0")])
        self.assertEqual([line for line in lines if not line.startswith("%")],
                         ["skip & 1.0 & 2.0 & collect & 3.0 & 6.0 \\\\", "fold & 4.0 & 5.0 \\\\"])


class Iterators(unittest.TestCase):
    """tab:scalability-iterators."""

    @staticmethod
    def rows(*cells):
        rows = [{"id": i, "setting": st, "side": sd, "run": "1", "verdict": v, "wall_s": t}
                for i, st, sd, v, t in cells]
        return tables.Rows(rows, {(i, st) for i, st, sd, _, _ in cells if sd == "fail"})

    def test_a_row_prints_written_and_inferred_in_each_setting_and_a_dash_where_nothing_ran(self):
        rows = self.rows(("tagged_take_assert_k1", "generic", "pass", "sat", "0.5"),
                         ("tagged_take_assert_k1", "generic", "fail", "unsat", "1.0"),
                         ("tagged_take_inferred_assert_k1", "per_instance", "pass", "timeout", "300.00"),
                         ("tagged_take_inferred_assert_k1", "per_instance", "fail", "unsat", "2.0"))
        body = tables.iterators_table(rows)
        self.assertEqual(body[0], "\\hiderowcolors\\multicolumn{6}{l}{\\texttt{k-types-take}} & "
                                  "\\multicolumn{6}{l}{\\texttt{k-types-skip}} \\\\")
        self.assertEqual(body[1], "\\stripedbody")
        self.assertEqual(body[2], "1 & 37 & 0.5 & -- & -- & t/o & 1 & 37 & -- & -- & -- & -- \\\\")

    def test_the_second_pair_is_the_figures_depth_benchmarks(self):
        self.assertEqual(tables.iterators_table(self.rows())[8:10],
                         ["\\midrule", "\\hiderowcolors\\multicolumn{6}{l}{\\texttt{k-chain-take}} & "
                                       "\\multicolumn{6}{l}{\\texttt{k-chain-skip}} \\\\"])

    def test_a_sat_pass_side_whose_twin_got_no_answer_prints_its_time(self):
        rows = self.rows(("tagged_take_inferred_assert_k1", "generic", "pass", "sat", "3.84"),
                         ("tagged_take_inferred_assert_k1", "generic", "fail", "timeout", "300.00"))
        self.assertEqual(tables.iterators_table(rows)[2],
                         "1 & 37 & -- & -- & 3.8 & -- & 1 & 37 & -- & -- & -- & -- \\\\")


class Markers(unittest.TestCase):
    """Every setting has a marker and is drawn once."""

    def test_the_settings_the_markers_and_the_drawing_order_agree(self):
        self.assertEqual(sorted(tables.DRAWN), sorted(tables.SETTINGS))
        self.assertEqual(sorted(tables.MARKS), sorted(tables.SETTINGS))


class FamiliesFigure(unittest.TestCase):
    """fig:scalability-families reads the pass sides of a family directly."""

    def test_a_point_its_list_does_not_mark_printed_stops_it(self):
        rows = tables.Rows([{"id": "base_k1", "setting": "generic", "side": "pass", "run": "1", "verdict": "sat",
                             "wall_s": "1.0"}], set(), {("base_k1", "generic", "pass")})
        with self.assertRaises(SystemExit):
            tables.family_points(rows, "base", "generic")


class IteratorsFigure(unittest.TestCase):
    """fig:scalability-iterators: the inferred cells per family, a cell without a verified time an open marker at the limit."""

    @staticmethod
    def rows(*cells, limit="120"):
        rows = [{"id": i, "setting": st, "side": sd, "run": "1", "verdict": v, "wall_s": t, "timeout_s": limit}
                for i, st, sd, v, t in cells]
        return tables.Rows(rows, {(i, st) for i, st, sd, _, _ in cells if sd == "fail"})

    def test_verified_cells_are_joined_a_cell_that_is_not_is_an_open_marker_and_a_size_without_rows_is_left_out(self):
        cells = []
        for k, verdict in ((1, "sat"), (2, "sat"), (4, "timeout"), (8, "sat")):
            cells += [(f"tagged_take_inferred_assert_k{k}", "per_instance", "pass", verdict, f"{k}.04"),
                      (f"tagged_take_inferred_assert_k{k}", "per_instance", "fail", "unsat", "1.0")]
        lines = tables.iterators_figure(self.rows(*cells))
        plots = [line for line in lines if line.startswith("\\addplot")]
        self.assertEqual(plots, [
            "\\addplot[forget plot, color=cper_instance, mark=square*, mark size=2pt, mark options={rotate=45}, dashed] coordinates {(1,1.0) (2,2.0)};",
            "\\addplot[forget plot, color=cper_instance, only marks, mark=square, mark size=2pt, mark options={rotate=45}, thick] coordinates {(4,120)};",
            "\\addplot[forget plot, color=cper_instance, mark=square*, mark size=2pt, mark options={rotate=45}, dashed] coordinates {(8,8.0)};"])
        for entry in ("generic, verified", "generic, timeout/out of memory", "per instance, verified",
                      "per instance, timeout/out of memory"):
            self.assertIn(f"\\addlegendentry{{{entry}}}", lines)
        self.assertIn("\\addlegendimage{color=cgeneric, only marks, mark=square, mark size=2pt, thick}", lines)
        self.assertEqual([line.count("ylabel=") for line in lines if line.startswith("\\nextgroupplot")], [1, 0, 1, 0])
        self.assertIn(tables.limit_line(120), lines)
        self.assertTrue(any(f"ymax={120 * tables.LIMIT_HEADROOM}," in line for line in lines))

    def test_out_of_memory_shares_the_timeout_marker_and_a_size_not_run_has_none(self):
        cells = []
        for k, verdict in ((1, "sat"), (2, "oom"), (4, "skipped")):
            cells += [(f"tagged_take_inferred_assert_k{k}", "per_instance", "pass", verdict, f"{k}.04")]
        lines = tables.iterators_figure(self.rows(*cells))
        self.assertEqual([line for line in lines if line.startswith("\\addplot")], [
            "\\addplot[forget plot, color=cper_instance, mark=square*, mark size=2pt, mark options={rotate=45}, dashed] coordinates {(1,1.0)};",
            "\\addplot[forget plot, color=cper_instance, only marks, mark=square, mark size=2pt, mark options={rotate=45}, thick] coordinates {(2,120)};"])
        self.assertIn("\\addlegendentry{per instance, timeout/out of memory}", lines)

    def test_a_cell_not_run_prints_a_dash_and_a_timeout_t_o(self):
        self.assertEqual(tables.scalability_text(tables.Cell("skipped", None, 0, ["1"], "skipped")), "--")
        self.assertEqual(tables.scalability_text(tables.Cell("timeout", None, 0, ["1"], "timeout")), "t/o")
        self.assertEqual(tables.scalability_text(tables.Cell("OOM", None, 0, ["1"], "oom")), "OOM")

    def test_a_cell_verified_in_some_runs_has_its_n_of_n_beside_its_marker(self):
        cells = []
        for k, verdicts in ((1, ("sat", "sat")), (2, ("sat", "timeout"))):
            for run, verdict in enumerate(verdicts, 1):
                cells += [(f"tagged_skip_inferred_assert_k{k}", "generic", "pass", verdict, f"{k}.0", str(run)),
                          (f"tagged_skip_inferred_assert_k{k}", "generic", "fail", "unsat", "1.0", str(run))]
        rows = tables.Rows([{"id": i, "setting": st, "side": sd, "run": n, "verdict": v, "wall_s": t,
                             "timeout_s": "120"} for i, st, sd, v, t, n in cells],
                           {("tagged_skip_inferred_assert_k1", "generic"), ("tagged_skip_inferred_assert_k2", "generic")})
        lines = tables.iterators_figure(rows)
        self.assertEqual([line for line in lines if line.startswith("\\addplot")], [
            "\\addplot[forget plot, color=cgeneric, mark=square*, mark size=2pt, solid] coordinates {(1,1.0) (2,2.0)};"])
        self.assertEqual([line for line in lines if line.startswith("\\node")], [
            "\\node[font=\\tiny, color=cgeneric, anchor=south west, inner sep=1pt] at (axis cs:2,2.0) {1/2};"])

    def test_the_written_variant_is_not_drawn(self):
        lines = tables.iterators_figure(self.rows(("tagged_take_assert_k1", "generic", "pass", "sat", "1.0")))
        self.assertFalse([line for line in lines if line.startswith("\\addplot")])

    def test_rows_with_two_time_limits_stop_it(self):
        rows = self.rows(("tagged_take_assert_k1", "generic", "pass", "sat", "1.0"))
        rows.append(dict(rows[0], timeout_s="300"))
        with self.assertRaises(SystemExit):
            tables.iterators_figure(rows)


if __name__ == "__main__":
    unittest.main()

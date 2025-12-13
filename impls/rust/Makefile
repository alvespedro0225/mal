EXEC_DIR := target/release

UPPER_STEPS := $(EXEC_DIR)/step4_if_fn_do \
               $(EXEC_DIR)/step5_tco \
               $(EXEC_DIR)/step6_file \
               $(EXEC_DIR)/step7_quote \
               $(EXEC_DIR)/step8_macros \
               $(EXEC_DIR)/step9_try \
               $(EXEC_DIR)/stepA_mal
STEP0       := $(EXEC_DIR)/step0_repl
STEP1-2     := $(EXEC_DIR)/step1_read_print \
               $(EXEC_DIR)/step2_eval
STEP3       := $(EXEC_DIR)/step3_env
STEPS := $(STEP0) $(STEP1-2) $(STEP3) $(UPPER_STEPS)

$(STEPS): $(EXEC_DIR)/%: %.rs

step0_repl.rs:
	cargo build --release --bin step0_repl

step1_read_print.rs:
	cargo build --release --bin step1_read_print

step2_eval.rs:
	cargo build --release --bin step2_eval

clean:
	cargo clean

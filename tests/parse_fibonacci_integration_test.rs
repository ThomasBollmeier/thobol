use std::process::Command;

#[test]
fn parse_fibonacci_file_to_expected_xml() {
    let source_path = format!("{}/tests/data/fibonacci.thobol", env!("CARGO_MANIFEST_DIR"));

    let output = Command::new(env!("CARGO_BIN_EXE_thobol"))
        .arg(source_path)
        .arg("--parse")
        .output()
        .expect("failed to execute thobol binary");

    assert!(
        output.status.success(),
        "expected parse command to succeed, but it failed.\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is not valid UTF-8");
    let expected = r#"<program>
  <var_definition>
    <name> fibonacci </name>
    <lambda_expression>
      <parameters>
        <parameter> n </parameter>
      </parameters>
      <body>
        <var_definition>
          <name> helper </name>
          <lambda_expression>
            <parameters>
              <parameter> a </parameter>
              <parameter> b </parameter>
              <parameter> idx </parameter>
            </parameters>
            <body>
              <if_expression>
                <comparison> =
                  <IDENTIFIER> idx </IDENTIFIER>
                  <IDENTIFIER> n </IDENTIFIER>
                </comparison>
                <then_branch>
                  <IDENTIFIER> a </IDENTIFIER>
                </then_branch>
                <else_branch>
                  <call>
                    <IDENTIFIER> helper </IDENTIFIER>
                    <arguments>
                      <IDENTIFIER> b </IDENTIFIER>
                      <add>
                        <IDENTIFIER> a </IDENTIFIER>
                        <IDENTIFIER> b </IDENTIFIER>
                      </add>
                      <add>
                        <IDENTIFIER> idx </IDENTIFIER>
                        <INTEGER> 1 </INTEGER>
                      </add>
                    </arguments>
                  </call>
                </else_branch>
              </if_expression>
            </body>
          </lambda_expression>
        </var_definition>
        <call>
          <IDENTIFIER> helper </IDENTIFIER>
          <arguments>
            <INTEGER> 1 </INTEGER>
            <INTEGER> 1 </INTEGER>
            <INTEGER> 0 </INTEGER>
          </arguments>
        </call>
      </body>
    </lambda_expression>
  </var_definition>
  <call>
    <IDENTIFIER> writeln </IDENTIFIER>
    <arguments>
      <call>
        <IDENTIFIER> fibonacci </IDENTIFIER>
        <arguments>
          <INTEGER> 10 </INTEGER>
        </arguments>
      </call>
    </arguments>
  </call>
</program>
"#;

    assert_eq!(stdout.trim_end(), expected.trim_end());
}
